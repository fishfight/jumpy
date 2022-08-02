use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

use hecs::{Entity, World};

use core::Transform;

use crate::game::play_sound_effect;
use crate::player::{
    Player, PlayerAttributes, PlayerController, PlayerEventQueue, JUMP_SOUND_ID, LAND_SOUND_ID,
    RESPAWN_DELAY,
};
use crate::{CollisionWorld, Drawable, DrawableKind, Item, Map, PhysicsBody, PlayerEvent};

const SLIDE_STOP_THRESHOLD: f32 = 2.0;
const JUMP_FRAME_COUNT: u16 = 8;
const PLATFORM_JUMP_FORCE_MULTIPLIER: f32 = 0.2;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlayerState {
    None,
    Jumping,
    Floating,
    Crouching,
    Sliding,
    Incapacitated,
    Dead,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::None
    }
}

pub fn update_player_states(world: &mut World) {
    let query = world.query_mut::<(
        &mut Transform,
        &mut Player,
        &PlayerController,
        &PlayerAttributes,
        &mut PhysicsBody,
    )>();
    for (_, (transform, player, controller, attributes, body)) in query {
        // Timers
        let dt = get_frame_time();

        player.attack_timer -= dt;
        if player.attack_timer <= 0.0 {
            player.attack_timer = 0.0;
        }

        player.is_attacking = player.attack_timer > 0.0;

        player.pickup_grace_timer += dt;

        if player.state == PlayerState::Crouching && !controller.should_crouch {
            player.state = PlayerState::None;
        }

        if player.state == PlayerState::Dead {
            player.respawn_timer += dt;

            player.passive_effects.clear();

            if player.respawn_timer >= RESPAWN_DELAY {
                player.state = PlayerState::None;
                player.respawn_timer = 0.0;

                let map = storage::get::<Map>();
                transform.position = map.get_random_spawn_point();
            }
        } else if player.state == PlayerState::Incapacitated {
            player.incapacitation_timer += dt;

            if player.incapacitation_timer >= attributes.incapacitation_duration {
                player.state = PlayerState::None;
                player.incapacitation_timer = 0.0;
            }
        }

        if player.state == PlayerState::Sliding && body.velocity.x.abs() <= SLIDE_STOP_THRESHOLD {
            body.velocity.x = 0.0;
            player.state = PlayerState::None;
        }

        // Integration
        if player.is_attacking
            || matches!(
                player.state,
                PlayerState::Dead | PlayerState::Incapacitated | PlayerState::Sliding
            )
        {
            body.has_friction = true;

            player.jump_frame_counter = 0;
            body.has_mass = true;
        } else {
            body.has_friction = false;

            if controller.move_direction.x < 0.0 {
                player.is_facing_left = true;
            } else if controller.move_direction.x > 0.0 {
                player.is_facing_left = false;
            }

            if controller.should_slide {
                let velocity = attributes.move_speed * attributes.slide_speed_factor;

                if player.is_facing_left {
                    body.velocity.x = -velocity;
                } else {
                    body.velocity.x = velocity;
                }

                player.state = PlayerState::Sliding;
            } else {
                if controller.move_direction.x < 0.0 {
                    body.velocity.x = -attributes.move_speed;
                } else if controller.move_direction.x > 0.0 {
                    body.velocity.x = attributes.move_speed;
                } else {
                    body.velocity.x = 0.0;
                }

                if controller.should_crouch {
                    if body.is_on_ground {
                        body.velocity.x = 0.0;
                        player.state = PlayerState::Crouching;
                    } else {
                        let mut collision_world = storage::get_mut::<CollisionWorld>();
                        collision_world.descent(body.actor);
                    }
                }

                if body.is_on_ground && controller.should_jump {
                    let jump_force = if controller.should_crouch && body.is_on_platform {
                        attributes.jump_force * PLATFORM_JUMP_FORCE_MULTIPLIER
                    } else {
                        attributes.jump_force
                    };

                    body.velocity.y = -jump_force;

                    player.state = PlayerState::Jumping;

                    play_sound_effect(JUMP_SOUND_ID, 0.4);
                } else if player.state == PlayerState::Jumping {
                    player.jump_frame_counter += 1;

                    if controller.should_float && player.jump_frame_counter <= JUMP_FRAME_COUNT {
                        body.has_mass = false;
                    } else {
                        if matches!(player.state, PlayerState::Jumping | PlayerState::Floating) {
                            player.state = PlayerState::None;
                        }

                        player.jump_frame_counter = 0;
                        body.has_mass = true;
                    }
                }

                if !body.is_on_ground && body.velocity.y > 0.0 {
                    if controller.should_float {
                        body.velocity.y *= attributes.float_gravity_factor;
                        player.state = PlayerState::Floating;
                    }
                } else if player.state == PlayerState::Floating {
                    player.state = PlayerState::None;
                }
            }

            // Note we can't use `body.was_on_ground` because it is only updated on fixed update,
            // and this function may run multiple times in one fixed update, resulting in the
            // landing sound being played more than once.
            if body.is_on_ground && !player.was_on_ground {
                if matches!(player.state, PlayerState::Jumping | PlayerState::Floating) {
                    player.state = PlayerState::None;
                }

                play_sound_effect(LAND_SOUND_ID, 0.4);

                player.jump_frame_counter = 0;
                body.has_mass = true;
            }
        }

        player.was_on_ground = body.is_on_ground;
    }
}

pub fn update_player_passive_effects(world: &mut World) {
    let mut function_calls = Vec::new();

    let mut sprites_to_spawn = Vec::new();
    let mut sprites_to_despawn = Vec::new();

    for (entity, (player, player_transform, player_drawable, events)) in world
        .query::<(&mut Player, &Transform, &Drawable, &mut PlayerEventQueue)>()
        .iter()
    {
        let dt = get_frame_time();

        for effect in &mut player.passive_effects {
            effect.duration_timer += dt;

            // Move the sprite to follow the player
            if let Some(sprite_entity) = effect.sprite_entity {
                let mut transform = world.get_mut::<Transform>(sprite_entity).unwrap();
                transform.position = player_transform.position;
            }

            // Spawn the effect sprite if it hasn't been spawned yet
            if let Some(sprite) = effect.sprite.take() {
                let sprite_entity = world.reserve_entity();

                let drawable = Drawable {
                    draw_order: player_drawable.draw_order + 1,
                    kind: DrawableKind::AnimatedSprite(sprite),
                };

                sprites_to_spawn.push((sprite_entity, drawable, player_transform.position));

                effect.sprite_entity = Some(sprite_entity);
            }
        }

        player.passive_effects.retain(|effect| {
            if effect.is_depleted() {
                if let Some(sprite_entity) = effect.sprite_entity {
                    sprites_to_despawn.push(sprite_entity);
                }

                false
            } else {
                true
            }
        });

        events.queue.push(PlayerEvent::Update { dt });

        for event in events.queue.iter() {
            let kind = event.into();

            for effect in &mut player.passive_effects {
                if effect.activated_on.contains(&kind) {
                    effect.use_cnt += 1;

                    if let Some(item_entity) = effect.item {
                        let mut item = world.get_mut::<Item>(item_entity).unwrap();

                        item.use_cnt += 1;
                    }

                    if let Some(f) = &effect.function {
                        function_calls.push((*f, entity, effect.item, event.clone()));
                    }
                }
            }
        }
    }

    for (f, player_entity, item_entity, event) in function_calls.drain(0..) {
        f(world, player_entity, item_entity, event);
    }

    for (sprite_entity, drawable, position) in sprites_to_spawn {
        world
            .insert(sprite_entity, (Transform::from(position), drawable))
            .unwrap();
    }

    for entity in sprites_to_despawn {
        world.despawn(entity).unwrap();
    }
}

pub fn on_player_damage(world: &mut World, damage_from_entity: Entity, damage_to_entity: Entity) {
    let mut is_from_left = false;

    if let Ok(owner_transform) = world.get::<Transform>(damage_from_entity) {
        if let Ok(target_transform) = world.get::<Transform>(damage_to_entity) {
            is_from_left = owner_transform.position.x < target_transform.position.x;
        }
    }

    {
        let mut events = world
            .get_mut::<PlayerEventQueue>(damage_from_entity)
            .unwrap();

        events.queue.push(PlayerEvent::GiveDamage {
            damage_to: Some(damage_to_entity),
        });
    }

    {
        let mut events = world.get_mut::<PlayerEventQueue>(damage_to_entity).unwrap();

        events.queue.push(PlayerEvent::ReceiveDamage {
            is_from_left,
            damage_from: Some(damage_from_entity),
        });
    }
}
