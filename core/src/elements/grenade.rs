use crate::prelude::*;
use std::time::Duration;

pub fn install(session: &mut CoreSession) {
    session
        .stages
        .add_system_to_stage(CoreStage::PreUpdate, hydrate)
        .add_system_to_stage(CoreStage::PostUpdate, update_lit_grenades)
        .add_system_to_stage(CoreStage::PostUpdate, update_idle_grenades);
}

#[derive(Clone, TypeUlid, Debug, Copy)]
#[ulid = "01GPRSBWQ3X0QJC37BDDQXDN84"]
pub struct IdleGrenade;

// #[derive(Clone, TypeUlid, Debug, Copy)]
#[derive(Clone, TypeUlid, Debug)]
#[ulid = "01GPY9N9CBR6EFJX0RS2H2K58J"]
pub struct LitGrenade {
    /// The owner of the grenade.
    pub owner: Entity,
    /// The amount of time left until the grenade explodes.
    pub fuse_time: Timer,
}

fn hydrate(
    game_meta: Res<CoreMetaArc>,
    mut entities: ResMut<Entities>,
    mut hydrated: CompMut<MapElementHydrated>,
    mut element_handles: CompMut<ElementHandle>,
    element_assets: BevyAssets<ElementMeta>,
    mut idle_grenades: CompMut<IdleGrenade>,
    mut atlas_sprites: CompMut<AtlasSprite>,
    mut animated_sprites: CompMut<AnimatedSprite>,
    mut bodies: CompMut<KinematicBody>,
    mut transforms: CompMut<Transform>,
    mut items: CompMut<Item>,
    mut item_throws: CompMut<ItemThrow>,
    mut item_grabs: CompMut<ItemGrab>,
    mut respawn_points: CompMut<DehydrateOutOfBounds>,
    mut spawner_manager: SpawnerManager,
) {
    let mut not_hydrated_bitset = hydrated.bitset().clone();
    not_hydrated_bitset.bit_not();
    not_hydrated_bitset.bit_and(element_handles.bitset());

    let spawner_entities = entities
        .iter_with_bitset(&not_hydrated_bitset)
        .collect::<Vec<_>>();

    for spawner_ent in spawner_entities {
        let transform = *transforms.get(spawner_ent).unwrap();
        let element_handle = element_handles.get(spawner_ent).unwrap();
        let Some(element_meta) = element_assets.get(&element_handle.get_bevy_handle()) else {
            continue;
        };

        if let BuiltinElementKind::Grenade {
            atlas,
            fin_anim,
            grab_offset,
            body_diameter,
            can_rotate,
            bounciness,
            throw_velocity,
            angular_velocity,
            ..
        } = &element_meta.builtin
        {
            hydrated.insert(spawner_ent, MapElementHydrated);

            let entity = entities.create();
            items.insert(entity, Item);
            idle_grenades.insert(entity, IdleGrenade);
            item_throws.insert(
                entity,
                ItemThrow::strength(*throw_velocity).with_spin(*angular_velocity),
            );
            item_grabs.insert(
                entity,
                ItemGrab {
                    fin_anim: *fin_anim,
                    sync_animation: false,
                    grab_offset: *grab_offset,
                },
            );
            atlas_sprites.insert(entity, AtlasSprite::new(atlas.clone()));
            respawn_points.insert(entity, DehydrateOutOfBounds(spawner_ent));
            transforms.insert(entity, transform);
            element_handles.insert(entity, element_handle.clone());
            hydrated.insert(entity, MapElementHydrated);
            animated_sprites.insert(entity, default());
            bodies.insert(
                entity,
                KinematicBody {
                    shape: ColliderShape::Circle {
                        diameter: *body_diameter,
                    },
                    has_mass: true,
                    has_friction: true,
                    can_rotate: *can_rotate,
                    bounciness: *bounciness,
                    gravity: game_meta.physics.gravity,
                    ..default()
                },
            );
            spawner_manager.create_spawner(spawner_ent, vec![entity])
        }
    }
}

fn update_idle_grenades(
    mut commands: Commands,
    entities: Res<Entities>,
    items_used: Comp<ItemUsed>,
    element_handles: Comp<ElementHandle>,
    element_assets: BevyAssets<ElementMeta>,
    mut audio_events: ResMut<AudioEvents>,
    mut idle_grenades: CompMut<IdleGrenade>,
    mut animated_sprites: CompMut<AnimatedSprite>,
) {
    for (entity, (_grenade, element_handle)) in
        entities.iter_with((&mut idle_grenades, &element_handles))
    {
        let Some(element_meta) = element_assets.get(&element_handle.get_bevy_handle()) else {
            continue;
        };

        let BuiltinElementKind::Grenade {
            fuse_sound,
            fuse_sound_volume,
            fuse_time,
            ..
        } = &element_meta.builtin else {
            unreachable!();
        };
        let fuse_time = *fuse_time;

        if items_used.get(entity).is_some() {
            // Animate Grenade
            let animated_sprite = animated_sprites.get_mut(entity).unwrap();
            animated_sprite.frames = Arc::from([3, 4, 5]);
            animated_sprite.repeat = true;
            animated_sprite.fps = 8.0;

            // Play that hissss sound
            audio_events.play(fuse_sound.clone(), *fuse_sound_volume);

            commands.add(
                move |mut lit: CompMut<LitGrenade>,
                      mut idle: CompMut<IdleGrenade>,
                      mut items_used: CompMut<ItemUsed>| {
                    idle.remove(entity);

                    lit.insert(
                        entity,
                        LitGrenade {
                            owner: items_used.get(entity).unwrap().owner,
                            fuse_time: Timer::new(
                                Duration::from_secs_f32(fuse_time),
                                TimerMode::Once,
                            ),
                        },
                    );

                    items_used.remove(entity);
                },
            );
        }
    }
}

fn update_lit_grenades(
    time: Res<Time>,
    mut commands: Commands,
    entities: Res<Entities>,
    transforms: CompMut<Transform>,
    element_handles: Comp<ElementHandle>,
    spawners: Comp<DehydrateOutOfBounds>,
    mut audio_events: ResMut<AudioEvents>,
    mut lit_grenades: CompMut<LitGrenade>,
    player_inventories: PlayerInventories,
    element_assets: BevyAssets<ElementMeta>,
    mut emote_regions: CompMut<EmoteRegion>,
    mut player_layers: CompMut<PlayerLayers>,
    mut hydrated: CompMut<MapElementHydrated>,
    mut trauma_events: ResMut<CameraTraumaEvents>,
) {
    for (entity, (grenade, element_handle, spawner)) in
        entities.iter_with((&mut lit_grenades, &element_handles, &spawners))
    {
        let Some(element_meta) = element_assets.get(&element_handle.get_bevy_handle()) else {
            continue;
        };

        let BuiltinElementKind::Grenade {
            explosion_sound,
            explosion_volume,
            damage_region_lifetime,
            damage_region_size,
            explosion_lifetime,
            explosion_atlas,
            explosion_fps,
            explosion_frames,
            fin_anim,
            ..
        } = &element_meta.builtin else {
            unreachable!();
        };

        grenade.fuse_time.tick(time.delta());

        if !emote_regions.contains(entity) {
            emote_regions.insert(
                entity,
                EmoteRegion {
                    active: true,
                    emote: Emote::Alarm,
                    owner: Some(grenade.owner),
                    direction_sensitive: true,
                    size: *damage_region_size * 2.0,
                    buffer: Some(Timer::new(Duration::from_millis(400), TimerMode::Once)),
                },
            );
        }
        let emote_region = emote_regions.get_mut(entity).unwrap();

        // If the item is being held
        if let Some(inventory) = player_inventories
            .iter()
            .find_map(|x| x.filter(|x| x.inventory == entity))
        {
            let player = inventory.player;
            let layers = player_layers.get_mut(player).unwrap();
            layers.fin_anim = *fin_anim;

            emote_region.active = false;

        // If the item is not being held
        } else {
            emote_region.active = true;
        }

        // If it's time to explode
        if grenade.fuse_time.finished() {
            audio_events.play(explosion_sound.clone(), *explosion_volume);

            trauma_events.send(5.0);

            // Cause the item to respawn by un-hydrating it's spawner.
            hydrated.remove(**spawner);
            let mut explosion_transform = *transforms.get(entity).unwrap();
            explosion_transform.translation.z = -10.0; // On top of almost everything
            explosion_transform.rotation = Quat::IDENTITY;

            // Clone types for move into closure
            let damage_region_size = *damage_region_size;
            let damage_region_lifetime = *damage_region_lifetime;
            let explosion_lifetime = *explosion_lifetime;
            let explosion_atlas = explosion_atlas.clone();
            let explosion_fps = *explosion_fps;
            let explosion_frames = *explosion_frames;
            commands.add(
                move |mut entities: ResMut<Entities>,
                      mut transforms: CompMut<Transform>,
                      mut damage_regions: CompMut<DamageRegion>,
                      mut lifetimes: CompMut<Lifetime>,
                      mut sprites: CompMut<AtlasSprite>,
                      mut animated_sprites: CompMut<AnimatedSprite>| {
                    // Despawn the grenade
                    entities.kill(entity);

                    // Spawn the damage region
                    let ent = entities.create();
                    transforms.insert(ent, explosion_transform);
                    damage_regions.insert(
                        ent,
                        DamageRegion {
                            size: damage_region_size,
                        },
                    );
                    lifetimes.insert(ent, Lifetime::new(damage_region_lifetime));

                    // Spawn the explosion animation
                    let ent = entities.create();
                    transforms.insert(ent, explosion_transform);
                    sprites.insert(
                        ent,
                        AtlasSprite {
                            atlas: explosion_atlas.clone(),
                            ..default()
                        },
                    );
                    animated_sprites.insert(
                        ent,
                        AnimatedSprite {
                            frames: (0..explosion_frames).collect(),
                            fps: explosion_fps,
                            repeat: false,
                            ..default()
                        },
                    );
                    lifetimes.insert(ent, Lifetime::new(explosion_lifetime));
                },
            );
        }
    }
}
