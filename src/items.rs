//! Things available to spawn from the level editor
//! Proto-mods, eventually some of the items will move to some sort of a wasm runtime

use hecs::{Entity, World};
use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

use serde::{Deserialize, Serialize};

use crate::game::sound::SOUND_EFFECT_VOLUME;
use crate::utils::timer::Timer;
use crate::{
    ActiveEffectMetadata, AnimatedSprite, AnimatedSpriteMetadata, CollisionWorld, Drawable, Owner,
    PassiveEffectMetadata, PhysicsBody, QueuedAnimationAction, Resources,
};

use core::{Result, Transform};

use crate::effects::active::spawn_active_effect;
use crate::particles::{ParticleEmitter, ParticleEmitterMetadata};
use crate::physics::PhysicsBodyParams;
use crate::player::{Player, PlayerInventory, IDLE_ANIMATION_ID};

pub const ITEMS_DRAW_ORDER: u32 = 1;

pub const SPRITE_ANIMATED_SPRITE_ID: &str = "sprite";
pub const EFFECT_ANIMATED_SPRITE_ID: &str = "effect";

pub const GROUND_ANIMATION_ID: &str = "ground";
pub const ATTACK_ANIMATION_ID: &str = "attack";

/// This dictates what happens to an item when it is dropped, either manually or on death.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemDropBehavior {
    /// Clear all state and restore default parameters
    ClearState,
    /// Keep all state between pickups
    PersistState,
    /// Destroy the item on drop
    Destroy,
}

impl Default for ItemDropBehavior {
    fn default() -> Self {
        ItemDropBehavior::ClearState
    }
}

/// This dictates what happens to an item when it is depleted, either by exceeding its duration,
/// in the case of `Equipment`, or by depleting `uses`, if specified, in the case of a `Weapon`
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemDepleteBehavior {
    /// Keep the item on depletion (do nothing)
    Keep,
    /// Drop the item on depletion
    Drop,
    /// Destroy the item on depletion
    Destroy,
}

impl Default for ItemDepleteBehavior {
    fn default() -> Self {
        ItemDepleteBehavior::Keep
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MapItemKind {
    Weapon {
        #[serde(flatten)]
        meta: WeaponMetadata,
    },
    Item {
        #[serde(flatten)]
        meta: ItemMetadata,
    },
}

pub struct ItemParams {
    pub name: String,
    pub effects: Vec<PassiveEffectMetadata>,
    pub uses: Option<u32>,
    pub duration: Option<f32>,
    pub mount_offset: Vec2,
    pub drop_behavior: ItemDropBehavior,
    pub deplete_behavior: ItemDepleteBehavior,
    pub is_hat: bool,
    pub respawn_info: Option<RespawnInfo>,
}

#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub effects: Vec<PassiveEffectMetadata>,
    pub uses: Option<u32>,
    pub duration: Option<f32>,
    pub mount_offset: Vec2,
    pub drop_behavior: ItemDropBehavior,
    pub deplete_behavior: ItemDepleteBehavior,
    pub is_hat: bool,
    pub duration_timer: f32,
    pub use_cnt: u32,
    pub respawn_info: Option<RespawnInfo>,
}

impl Item {
    pub fn new(id: &str, params: ItemParams) -> Self {
        Item {
            id: id.to_string(),
            name: params.name,
            effects: params.effects,
            uses: params.uses,
            duration: params.duration,
            mount_offset: params.mount_offset,
            drop_behavior: params.drop_behavior,
            deplete_behavior: params.deplete_behavior,
            respawn_info: params.respawn_info,
            is_hat: params.is_hat,
            duration_timer: 0.0,
            use_cnt: 0,
        }
    }
}

/// This holds the parameters used when constructing an `Equipment`
#[derive(Clone, Serialize, Deserialize)]
pub struct ItemMetadata {
    /// The effects that will be instantiated when the item is equipped
    #[serde(default)]
    pub effects: Vec<PassiveEffectMetadata>,
    /// The items duration, after being equipped. This will also be the default duration of
    /// passive effects that are added to the player, when equipping the item
    #[serde(default)]
    pub duration: Option<f32>,
    /// If this is `true` the item will be treated as a hat
    #[serde(default, rename = "hat", skip_serializing_if = "core::json::is_false")]
    pub is_hat: bool,
}

// NOTE: We would prefer to `serde(deny_unknown_fields)` here, but we are blocked by this issue:
// https://github.com/serde-rs/serde/issues/1358
#[derive(Clone, Serialize, Deserialize)]
pub struct MapItemMetadata {
    pub id: String,
    pub name: String,
    #[serde(flatten)]
    pub kind: MapItemKind,
    #[serde(default)]
    pub can_rotate: bool,
    #[serde(with = "core::json::vec2_def")]
    pub collider_size: Vec2,
    #[serde(default, with = "core::json::vec2_def")]
    pub collider_offset: Vec2,
    #[serde(default)]
    pub uses: Option<u32>,
    #[serde(default)]
    pub drop_behavior: ItemDropBehavior,
    #[serde(default)]
    pub deplete_behavior: ItemDepleteBehavior,
    /// If specified, the item will be respawned if it is depleted or falls off the map, after the
    /// specified delay in seconds.
    #[serde(default = "default_respawn_delay")]
    pub respawn_delay: Option<f32>,
    /// This specifies the offset from the player position to where the equipped item is drawn
    #[serde(default, with = "core::json::vec2_def")]
    pub mount_offset: Vec2,
    /// The parameters for the `AnimationPlayer` that will be used to draw the item
    #[serde(alias = "animation")]
    pub sprite: AnimatedSpriteMetadata,
}

fn default_respawn_delay() -> Option<f32> {
    Some(3.0)
}

pub fn spawn_item(world: &mut World, position: Vec2, meta: MapItemMetadata) -> Result<Entity> {
    let mut sprites = Vec::new();

    let MapItemMetadata {
        collider_size,
        collider_offset,
        drop_behavior,
        deplete_behavior,
        mount_offset,
        ..
    } = meta;

    let actor = storage::get_mut::<CollisionWorld>().add_actor(
        position,
        collider_size.x as i32,
        collider_size.y as i32,
    );

    let sprite = meta.sprite.into();

    sprites.push((SPRITE_ANIMATED_SPRITE_ID, sprite));

    let id = meta.id.as_str();

    let entity = world.spawn((
        Transform::from(position),
        PhysicsBody::new(
            actor,
            None,
            PhysicsBodyParams {
                size: collider_size,
                offset: collider_offset,
                has_mass: true,
                has_friction: true,
                can_rotate: meta.can_rotate,
                ..Default::default()
            },
        ),
    ));

    let uses = meta.uses;
    let respawn_info = meta.respawn_delay.map(|respawn_delay| RespawnInfo {
        position,
        respawn_delay,
    });

    let name = meta.name.clone();

    match meta.kind {
        MapItemKind::Item { meta } => {
            let ItemMetadata {
                effects,
                duration,
                is_hat,
            } = meta;

            world.insert_one(
                entity,
                Item::new(
                    id,
                    ItemParams {
                        name,
                        effects,
                        uses,
                        duration,
                        mount_offset,
                        drop_behavior,
                        deplete_behavior,
                        is_hat,
                        respawn_info,
                    },
                ),
            )?;

            if !sprites.is_empty() {
                world.insert_one(
                    entity,
                    Drawable::new_animated_sprite_set(ITEMS_DRAW_ORDER, sprites.as_slice()),
                )?;
            }
        }
        MapItemKind::Weapon { meta } => {
            let effect_offset = meta.effect_offset;

            let mut sound_effect = None;
            if let Some(id) = meta.sound_effect_id.as_ref() {
                sound_effect = storage::get::<Resources>().sounds.get(id).copied();
            }

            if let Some(effect_sprite) = meta.effect_sprite {
                let mut sprite: AnimatedSprite = effect_sprite.into();
                sprite.is_deactivated = true;

                sprites.push((EFFECT_ANIMATED_SPRITE_ID, sprite));
            }

            let particle_emitters = meta
                .particles
                .clone()
                .into_iter()
                .map(ParticleEmitter::new)
                .collect::<Vec<_>>();

            if !particle_emitters.is_empty() {
                world.insert_one(entity, particle_emitters).unwrap();
            }

            let params = WeaponParams {
                name,
                effects: meta.effects,
                uses,
                sound_effect,
                mount_offset,
                effect_offset,
                drop_behavior,
                deplete_behavior,
                respawn_info,
            };

            world.insert_one(
                entity,
                Weapon::new(id, meta.recoil, meta.cooldown, meta.attack_duration, params),
            )?;

            if !sprites.is_empty() {
                world.insert_one(
                    entity,
                    Drawable::new_animated_sprite_set(ITEMS_DRAW_ORDER, sprites.as_slice()),
                )?;
            }
        }
    }

    Ok(entity)
}

pub struct WeaponParams {
    pub name: String,
    pub effects: Vec<ActiveEffectMetadata>,
    pub uses: Option<u32>,
    pub sound_effect: Option<Sound>,
    pub mount_offset: Vec2,
    pub effect_offset: Vec2,
    pub drop_behavior: ItemDropBehavior,
    pub deplete_behavior: ItemDepleteBehavior,
    pub respawn_info: Option<RespawnInfo>,
}

impl Default for WeaponParams {
    fn default() -> Self {
        WeaponParams {
            name: "".to_string(),
            effects: Vec::new(),
            uses: None,
            sound_effect: None,
            mount_offset: Vec2::ZERO,
            effect_offset: Vec2::ZERO,
            drop_behavior: Default::default(),
            deplete_behavior: Default::default(),
            respawn_info: None,
        }
    }
}

#[derive(Clone)]
pub struct Weapon {
    pub id: String,
    pub name: String,
    pub effects: Vec<ActiveEffectMetadata>,
    pub sound_effect: Option<Sound>,
    pub recoil: f32,
    pub cooldown: f32,
    pub attack_duration: f32,
    pub uses: Option<u32>,
    pub mount_offset: Vec2,
    pub effect_offset: Vec2,
    pub drop_behavior: ItemDropBehavior,
    pub deplete_behavior: ItemDepleteBehavior,
    pub cooldown_timer: f32,
    pub use_cnt: u32,
    pub respawn_info: Option<RespawnInfo>,
}

impl Weapon {
    pub fn new(
        id: &str,
        recoil: f32,
        cooldown: f32,
        attack_duration: f32,
        params: WeaponParams,
    ) -> Self {
        Weapon {
            id: id.to_string(),
            name: params.name,
            effects: params.effects,
            recoil,
            cooldown,
            uses: params.uses,
            attack_duration,
            sound_effect: params.sound_effect,
            mount_offset: params.mount_offset,
            effect_offset: params.effect_offset,
            drop_behavior: params.drop_behavior,
            deplete_behavior: params.deplete_behavior,
            respawn_info: params.respawn_info,
            cooldown_timer: cooldown,
            use_cnt: 0,
        }
    }
}

pub fn fire_weapon(world: &mut World, entity: Entity, owner: Entity) -> Result<()> {
    let mut effects = Vec::new();

    let mut origin = Vec2::ZERO;

    {
        let mut weapon = world.get_mut::<Weapon>(entity).unwrap();

        if weapon.cooldown_timer >= weapon.cooldown {
            let mut player = world.get_mut::<Player>(owner).unwrap();

            {
                let mut owner_body = world.get_mut::<PhysicsBody>(owner).unwrap();

                if player.is_facing_left {
                    owner_body.velocity.x = weapon.recoil;
                } else {
                    owner_body.velocity.x = -weapon.recoil;
                }

                let owner_transform = world.get::<Transform>(owner).unwrap();
                let owner_inventory = world.get::<PlayerInventory>(owner).unwrap();

                origin = owner_transform.position
                    + owner_inventory
                        .get_weapon_mount(player.is_facing_left, player.is_upside_down);

                let mut offset = weapon.mount_offset + weapon.effect_offset;
                if player.is_facing_left {
                    offset.x = -offset.x;
                }

                origin += offset;
            }

            player.attack_timer = weapon.attack_duration;

            weapon.use_cnt += 1;

            weapon.cooldown_timer = 0.0;

            if let Some(sound) = weapon.sound_effect {
                play_sound(
                    sound,
                    PlaySoundParams {
                        looped: false,
                        volume: SOUND_EFFECT_VOLUME,
                    },
                );
            }

            let mut drawable = world.get_mut::<Drawable>(entity).unwrap();
            {
                let sprite_set = drawable.get_animated_sprite_set_mut().unwrap();

                {
                    let sprite = sprite_set.map.get_mut(SPRITE_ANIMATED_SPRITE_ID).unwrap();
                    let is_looping = sprite
                        .get_animation(ATTACK_ANIMATION_ID)
                        .map(|a| a.is_looping)
                        .unwrap_or_default();

                    sprite.set_animation(ATTACK_ANIMATION_ID, !is_looping);
                    sprite.queue_action(QueuedAnimationAction::Play(IDLE_ANIMATION_ID.to_string()));
                }

                if let Some(sprite) = sprite_set.map.get_mut(EFFECT_ANIMATED_SPRITE_ID) {
                    sprite.is_deactivated = false;

                    let is_looping = sprite
                        .get_animation(ATTACK_ANIMATION_ID)
                        .map(|a| a.is_looping)
                        .unwrap_or_default();

                    sprite.set_animation(ATTACK_ANIMATION_ID, !is_looping);
                    sprite.queue_action(QueuedAnimationAction::Deactivate);
                }
            }

            if let Ok(mut particle_emitters) = world.get_mut::<Vec<ParticleEmitter>>(entity) {
                for emitter in particle_emitters.iter_mut() {
                    emitter.activate();
                }
            }

            effects = weapon.effects.clone();
        }
    }

    for params in effects {
        spawn_active_effect(world, owner, entity, origin, params)?;
    }

    Ok(())
}

/// This holds the parameters for the `AnimationPlayer` components of an equipped `Weapon`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponAnimationMetadata {
    /// This holds the parameters of the main `AnimationPlayer` component, holding the main
    /// animations, like `"idle"` and `"attack"`.
    /// At a minimum, an animation with the id `"idle"` must be specified. If no animation is
    /// required, an animation with one frame can be used to just display a sprite.
    #[serde(rename = "animation")]
    pub sprite: AnimatedSpriteMetadata,
    /// This can hold the parameters of the effect `AnimationPlayer` component, holding the
    /// animations used for effects like `"attack_effect"`.
    /// At a minimum, if this is specified, an animation with the id `"attack_effect"` must be
    /// specified. If no animation is required, an animation with one frame can be used to just
    /// display a sprite.
    #[serde(
        default,
        rename = "effect_animation",
        skip_serializing_if = "Option::is_none"
    )]
    pub effect: Option<AnimatedSpriteMetadata>,
}

/// This holds parameters specific to the `Weapon` variant of `ItemKind`, used to instantiate a
/// `Weapon` struct instance, when an `Item` of type `Weapon` is picked up.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeaponMetadata {
    /// This specifies the effects to instantiate when the weapon is used to attack
    #[serde(default)]
    pub effects: Vec<ActiveEffectMetadata>,
    /// Particle effects that will be activated when using the weapon
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub particles: Vec<ParticleEmitterMetadata>,
    /// This can specify an id of a sound effect that is played when the weapon is used to attack
    #[serde(
        default,
        rename = "sound_effect",
        skip_serializing_if = "Option::is_none"
    )]
    pub sound_effect_id: Option<String>,
    /// This specifies the offset between the upper left corner of the weapon's sprite to the
    /// position that will serve as the origin of the weapon's effects
    #[serde(default, with = "core::json::vec2_def")]
    pub effect_offset: Vec2,
    /// This can specify a maximum amount of weapon uses. If no value is specified, the weapon
    /// will have unlimited uses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uses: Option<u32>,
    /// This specifies the minimum interval of attacks with the weapon
    #[serde(default)]
    pub cooldown: f32,
    /// This specifies the amount of time the player will be locked in an attack state when using
    /// the weapon
    #[serde(default)]
    pub attack_duration: f32,
    /// This specifies the force applied to the `Player` velocity, in the opposite direction of the
    /// attack, when the weapon is activated.
    #[serde(default)]
    pub recoil: f32,
    /// This can hold the parameters of the effect `AnimationPlayer` component, holding the
    /// animations used for effects.
    /// At a minimum, if this is specified, an animation with the id `"attack"` must be
    /// specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect_sprite: Option<AnimatedSpriteMetadata>,
}

impl Default for WeaponMetadata {
    fn default() -> Self {
        WeaponMetadata {
            effects: Vec::new(),
            particles: Vec::new(),
            sound_effect_id: None,
            uses: None,
            effect_offset: Vec2::ZERO,
            cooldown: 0.0,
            attack_duration: 0.0,
            recoil: 0.0,
            effect_sprite: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RespawnInfo {
    pub position: Vec2,
    pub respawn_delay: f32,
}

/// An item that has been destroyed somehow that should respawn
#[derive(Clone)]
pub struct RespawningItem {
    pub timer: Timer,
    pub info: RespawnInfo,
    pub kind: RespawningItemKind,
}

#[derive(Clone)]
pub enum RespawningItemKind {
    Weapon(Weapon),
    Item(Item),
}

pub fn update_respawning_items(world: &mut World) {
    let mut to_spawn = Vec::new();

    for (respawning_item_entity, (respawning_item, transform, body, drawable)) in world
        .query_mut::<(
            &mut RespawningItem,
            &mut Transform,
            &mut PhysicsBody,
            &mut Drawable,
        )>()
    {
        let respawning_item: &mut RespawningItem = respawning_item;
        let transform: &mut Transform = transform;
        let body: &mut PhysicsBody = body;
        let drawable: &mut Drawable = drawable;

        respawning_item.timer.tick_frame_time();

        if respawning_item.timer.has_finished() {
            transform.position = respawning_item.info.position;
            body.velocity = Vec2::ZERO;
            body.is_deactivated = false;
            match &mut drawable.kind {
                crate::DrawableKind::Sprite(s) => s.is_deactivated = false,
                crate::DrawableKind::SpriteSet(s) => {
                    if let Some(sprite) = s.map.get_mut(SPRITE_ANIMATED_SPRITE_ID) {
                        sprite.is_deactivated = false;
                    }
                }
                crate::DrawableKind::AnimatedSprite(s) => s.is_deactivated = false,
                crate::DrawableKind::AnimatedSpriteSet(s) => {
                    if let Some(sprite) = s.map.get_mut(SPRITE_ANIMATED_SPRITE_ID) {
                        sprite.is_deactivated = false;
                    }
                }
            }
            to_spawn.push(respawning_item_entity);
        } else {
            match &mut drawable.kind {
                crate::DrawableKind::Sprite(s) => s.is_deactivated = true,
                crate::DrawableKind::SpriteSet(s) => s.deactivate_all(),
                crate::DrawableKind::AnimatedSprite(s) => s.is_deactivated = true,
                crate::DrawableKind::AnimatedSpriteSet(s) => s.deactivate_all(),
            }
        }
    }

    for entity in to_spawn {
        let respawning_item = world.remove_one::<RespawningItem>(entity).unwrap();
        world.remove_one::<Owner>(entity).ok();

        match respawning_item.kind {
            RespawningItemKind::Weapon(mut weapon) => {
                weapon.use_cnt = 0;
                world.insert_one(entity, weapon)
            }
            RespawningItemKind::Item(mut item) => {
                item.use_cnt = 0;
                item.duration_timer = 0.0;
                world.insert_one(entity, item)
            }
        }
        .unwrap();
    }
}
