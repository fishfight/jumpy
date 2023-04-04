use crate::prelude::*;

pub fn install(session: &mut CoreSession) {
    session
        .stages
        .add_system_to_stage(CoreStage::PreUpdate, hydrate)
        .add_system_to_stage(CoreStage::PostUpdate, update_thrown_mines)
        .add_system_to_stage(CoreStage::PostUpdate, update_idle_mines);
}

#[derive(Clone, TypeUlid, Debug, Copy)]
#[ulid = "01GPRSBWQ3X0QJC37BDDQXDNF3"]
pub struct IdleMine;

#[derive(Clone, TypeUlid, Debug, Copy)]
#[ulid = "01GPRSBWQ3X0QJC37BDQXDNASF"]
pub struct ThrownMine {
    // How long the mine has been thrown.
    age: f32,
}

fn hydrate(
    game_meta: Res<CoreMetaArc>,
    mut entities: ResMut<Entities>,
    mut hydrated: CompMut<MapElementHydrated>,
    mut element_handles: CompMut<ElementHandle>,
    element_assets: BevyAssets<ElementMeta>,
    mut idle_mines: CompMut<IdleMine>,
    mut atlas_sprites: CompMut<AtlasSprite>,
    mut animated_sprites: CompMut<AnimatedSprite>,
    mut bodies: CompMut<KinematicBody>,
    mut transforms: CompMut<Transform>,
    mut items: CompMut<Item>,
    mut item_throws: CompMut<ItemThrow>,
    mut item_grabs: CompMut<ItemGrab>,
    mut respawn_points: CompMut<DehydrateOutOfBounds>,
) {
    let mut not_hydrated_bitset = hydrated.bitset().clone();
    not_hydrated_bitset.bit_not();
    not_hydrated_bitset.bit_and(element_handles.bitset());

    let spawners = entities
        .iter_with_bitset(&not_hydrated_bitset)
        .collect::<Vec<_>>();

    for spawner_ent in spawners {
        let transform = *transforms.get(spawner_ent).unwrap();
        let element_handle = element_handles.get(spawner_ent).unwrap();
        let Some(element_meta) = element_assets.get(&element_handle.get_bevy_handle()) else {
            continue;
        };

        if let BuiltinElementKind::Mine {
            atlas,
            fin_anim,
            grab_offset,
            body_size,
            bounciness,
            throw_velocity,
            ..
        } = &element_meta.builtin
        {
            hydrated.insert(spawner_ent, MapElementHydrated);

            let entity = entities.create();
            items.insert(entity, Item);
            idle_mines.insert(entity, IdleMine);
            atlas_sprites.insert(entity, AtlasSprite::new(atlas.clone()));
            item_throws.insert(entity, ItemThrow::strength(*throw_velocity));
            item_grabs.insert(
                entity,
                ItemGrab {
                    fin_anim: *fin_anim,
                    sync_animation: false,
                    grab_offset: *grab_offset,
                },
            );
            respawn_points.insert(entity, DehydrateOutOfBounds(spawner_ent));
            transforms.insert(entity, transform);
            element_handles.insert(entity, element_handle.clone());
            hydrated.insert(entity, MapElementHydrated);
            animated_sprites.insert(entity, default());
            bodies.insert(
                entity,
                KinematicBody {
                    shape: ColliderShape::Rectangle { size: *body_size },
                    has_mass: true,
                    has_friction: true,
                    bounciness: *bounciness,
                    gravity: game_meta.physics.gravity,
                    ..default()
                },
            );
        }
    }
}

fn update_idle_mines(
    entities: Res<Entities>,
    mut idle_mines: CompMut<IdleMine>,
    mut items_used: CompMut<ItemUsed>,
    player_inventories: PlayerInventories,
    mut commands: Commands,
) {
    for (entity, _mine) in entities.iter_with(&mut idle_mines) {
        if let Some(Inv { player, .. }) = player_inventories
            .iter()
            .find_map(|x| x.filter(|x| x.inventory == entity))
        {
            if items_used.get(entity).is_some() {
                items_used.remove(entity);
                commands.add(PlayerCommand::set_inventory(player, None));
                commands.add(
                    move |mut idle: CompMut<IdleMine>, mut thrown: CompMut<ThrownMine>| {
                        idle.remove(entity);
                        thrown.insert(entity, ThrownMine { age: 0.0 });
                    },
                );
            }
        }
    }
}

fn update_thrown_mines(
    entities: Res<Entities>,
    element_handles: Comp<ElementHandle>,
    element_assets: BevyAssets<ElementMeta>,
    mut audio_events: ResMut<AudioEvents>,
    mut trauma_events: ResMut<CameraTraumaEvents>,
    mut thrown_mines: CompMut<ThrownMine>,
    mut animated_sprites: CompMut<AnimatedSprite>,
    mut hydrated: CompMut<MapElementHydrated>,
    player_indexes: Comp<PlayerIdx>,
    mut commands: Commands,
    collision_world: CollisionWorld,
    transforms: Comp<Transform>,
    spawners: Comp<DehydrateOutOfBounds>,
    invincibles: CompMut<Invincibility>,
) {
    let players = entities
        .iter_with(&player_indexes)
        .map(|x| x.0)
        .collect::<Vec<_>>();
    for (entity, (thrown_mine, element_handle, sprite, spawner)) in entities.iter_with((
        &mut thrown_mines,
        &element_handles,
        &mut animated_sprites,
        &spawners,
    )) {
        let Some(element_meta) = element_assets.get(&element_handle.get_bevy_handle()) else {
            continue;
        };

        let BuiltinElementKind::Mine {
            explosion_fps,
            explosion_frames,
            explosion_sound,
            explosion_atlas,
            arm_delay,
            arm_sound,
            armed_frames,
            armed_fps,
            damage_region_size,
            damage_region_lifetime, explosion_volume, arm_sound_volume, explosion_lifetime, .. } = &element_meta.builtin else {
            unreachable!();
        };

        let frame_time = 1.0 / crate::FPS;
        thrown_mine.age += 1.0 / crate::FPS;

        if thrown_mine.age >= *arm_delay && thrown_mine.age - *arm_delay < frame_time {
            audio_events.play(arm_sound.clone(), *arm_sound_volume);

            sprite.frames = (0..*armed_frames).collect();
            sprite.fps = *armed_fps;
            sprite.repeat = true;
        }

        let colliding_with_players = collision_world
            .actor_collisions_filtered(entity, |e| {
                players.contains(&e) && invincibles.get(e).is_none()
            })
            .into_iter()
            .collect::<Vec<_>>();

        if !colliding_with_players.is_empty() && thrown_mine.age >= *arm_delay {
            let mine_transform = *transforms.get(entity).unwrap();

            trauma_events.send(6.0);

            for player in &colliding_with_players {
                commands.add(PlayerCommand::kill(
                    *player,
                    Some(mine_transform.translation.xy()),
                ));
            }

            audio_events.play(explosion_sound.clone(), *explosion_volume);

            hydrated.remove(**spawner);

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
                    let mut explosion_transform = mine_transform;
                    explosion_transform.translation.z = -10.0; // On top of almost everything
                    explosion_transform.rotation = Quat::IDENTITY;

                    // Despawn the grenade
                    entities.kill(entity);

                    // Spawn the damage region
                    let damage_ent = entities.create();
                    transforms.insert(damage_ent, explosion_transform);
                    damage_regions.insert(
                        damage_ent,
                        DamageRegion {
                            size: damage_region_size,
                        },
                    );
                    lifetimes.insert(damage_ent, Lifetime::new(damage_region_lifetime));

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
