use crate::{
    animation::AnimatedSprite,
    damage::{DamageRegion, DamageRegionOwner},
    item::{Item, ItemDropped, ItemUsed},
    lifetime::Lifetime,
    map::{MapElementHydrated, MapRespawnPoint},
    metadata::{BuiltinElementKind, MapElementMeta},
    physics::{collisions::CollisionWorld, KinematicBody},
    player::{input::PlayerInputs, PlayerIdx, MAX_PLAYERS},
    prelude::*,
    utils::Sort,
};

// Meta/environment elements
pub mod crab;
pub mod decoration;
pub mod player_spawner;
pub mod sproinger;

// Items
pub mod crate_item;
pub mod grenade;
pub mod mine;
pub mod stomp_boots;
pub mod sword;

pub struct MapElementsPlugin;

impl Plugin for MapElementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(decoration::DecorationPlugin)
            .add_plugin(grenade::GrenadePlugin)
            .add_plugin(crate_item::CrateItemPlugin)
            .add_plugin(player_spawner::PlayerSpawnerPlugin)
            .add_plugin(sproinger::SproingerPlugin)
            .add_plugin(mine::MinePlugin)
            .add_plugin(stomp_boots::StompBootsPlugin)
            .add_plugin(sword::SwordPlugin)
            .add_plugin(crab::CrabDecorationPlugin);
    }
}
