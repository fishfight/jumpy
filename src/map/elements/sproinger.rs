use crate::metadata::BuiltinElementKind;

use super::*;

const FORCE: f32 = 30.0;

pub struct SproingerPlugin;
impl Plugin for SproingerPlugin {
    fn build(&self, app: &mut App) {
        app.extend_rollback_schedule(|schedule| {
            schedule
                .add_system_to_stage(RollbackStage::PreUpdateInGame, pre_update_in_game)
                .add_system_to_stage(RollbackStage::UpdateInGame, update_in_game);
        })
        .extend_rollback_plugin(|plugin| plugin.register_rollback_type::<Sproinger>());
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Sproinger {
    frame: u32,
    sproinging: bool,
}

fn pre_update_in_game(
    mut commands: Commands,
    non_hydrated_map_elements: Query<(Entity, &MapElementMeta), Without<MapElementHydrated>>,
) {
    // Hydrate any newly-spawned sproingers
    for (entity, map_element) in &non_hydrated_map_elements {
        if let BuiltinElementKind::Sproinger { atlas_handle, .. } = &map_element.builtin {
            commands
                .entity(entity)
                .insert(MapElementHydrated)
                .insert(Sproinger::default())
                .insert(AnimatedSprite {
                    start: 0,
                    end: 6,
                    atlas: atlas_handle.inner.clone(),
                    repeat: false,
                    fps: 0.0,
                    ..default()
                })
                .insert(KinematicBody {
                    size: Vec2::new(32.0, 8.0),
                    offset: Vec2::new(0.0, -6.0),
                    has_mass: false,
                    ..default()
                });
        }
    }
}

fn update_in_game(
    mut sproingers: Query<(Entity, &mut Sproinger, &mut AnimatedSprite)>,
    mut bodies: Query<&mut KinematicBody>,
    collision_world: CollisionWorld,
) {
    for (sproinger_ent, mut sproinger, mut sprite) in &mut sproingers {
        if sproinger.sproinging {
            match sproinger.frame {
                1 => sprite.index = 2,
                4 => sprite.index = 3,
                8 => sprite.index = 4,
                12 => sprite.index = 5,
                20 => {
                    sprite.index = 0;
                    sproinger.sproinging = false;
                    sproinger.frame = 0;
                }
                _ => (),
            }
            sproinger.frame += 1;
        }

        for collider_ent in collision_world.actor_collisions(sproinger_ent) {
            let mut body = bodies.get_mut(collider_ent).unwrap();

            if !sproinger.sproinging {
                body.velocity.y = FORCE;

                sproinger.sproinging = true;
            }
        }
    }
}
