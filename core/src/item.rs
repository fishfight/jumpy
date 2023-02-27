use crate::prelude::*;

pub fn install(session: &mut GameSession) {
    session
        .stages
        .add_system_to_stage(CoreStage::Last, throw_dropped_items);
}

/// Marker component for items.
///
/// Items are any entity that players can pick up and use.
#[derive(Clone, Copy, TypeUlid)]
#[ulid = "01GP4DBSEB3R6ZNBNNTSY36GW4"]
pub struct Item;

/// An intventory component, indicating another entity that the player is carrying.
#[derive(Clone, TypeUlid, Default, Deref, DerefMut)]
#[ulid = "01GP4D6M2QBSKZMEZMM22YGG41"]
pub struct Inventory(pub Option<Entity>);

/// A helper struct containing a player-inventory pair that indicates the given player is holding
/// the other entity in their inventory.
#[derive(Debug, Clone, Copy)]
pub struct Inv {
    pub player: Entity,
    pub inventory: Entity,
}

/// System param that can be used to conveniently get the inventory of each player.
#[derive(Deref, DerefMut, Debug)]
pub struct PlayerInventories<'a>(&'a [Option<Inv>; MAX_PLAYERS]);

impl<'a> SystemParam for PlayerInventories<'a> {
    type State = [Option<Inv>; MAX_PLAYERS];
    type Param<'s> = PlayerInventories<'s>;

    fn initialize(world: &mut World) {
        world.components.init::<Inventory>();
        world.components.init::<PlayerIdx>();
    }

    fn get_state(world: &World) -> Self::State {
        world
            .run_initialized_system(
                |entities: Res<Entities>,
                 player_indexes: Comp<PlayerIdx>,
                 inventories: Comp<Inventory>| {
                    let mut player_inventories = [None; MAX_PLAYERS];
                    for (player, (idx, inventory)) in
                        entities.iter_with((&player_indexes, &inventories))
                    {
                        if let Some(inventory) = inventory.0 {
                            player_inventories[idx.0] = Some(Inv { player, inventory });
                        }
                    }

                    Ok(player_inventories)
                },
            )
            .unwrap()
    }

    fn borrow<'s>(state: &mut Self::State) -> Self::Param<'_> {
        PlayerInventories(state)
    }
}

/// Marker component added to items when they are dropped.
#[derive(Clone, Copy, TypeUlid)]
#[ulid = "01GP4DH23M7M2CXVWADPZHW54F"]
pub struct ItemDropped {
    /// The player that dropped the item
    pub player: Entity,
}

/// Marker component added to items when they are grabbed.
#[derive(Clone, Copy, TypeUlid)]
#[ulid = "01GP4DJ2RPYTDPKSKEK8JKK9VT"]
pub struct ItemGrabbed;

/// Marker component added to items when they are used.
#[derive(Clone, Copy, TypeUlid)]
#[ulid = "01GP4DJ84TFB8Z7H9VY7Y0R47H"]
pub struct ItemUsed;

/// Component defining the strength of the throw types when an item is dropped.
///
/// Mainly handled by the [`throw_dropped_items`] system.
#[derive(Clone, Copy, TypeUlid)]
#[ulid = "01GSGE6N4TSEMQ1DKDP5Y66TE4"]
pub struct ItemThrow {
    normal: Vec2,
    fast: Vec2,
    up: Vec2,
    drop: Vec2,
    lob: Vec2,
    roll: Vec2,
    spin: f32,
}

impl ItemThrow {
    /// The relative velocities of each different throw type.
    ///
    /// This is multiiplied by the desired throw strength in [`Self::strength`] to get a deafault
    /// throw pattern from a single velocity.
    pub fn base() -> Self {
        Self {
            normal: Vec2::new(1.5, 1.2).normalize() * 0.6,
            fast: Vec2::new(1.5, 1.2).normalize(),
            up: Vec2::new(0.0, 1.1),
            drop: Vec2::new(0.0, 0.0),
            lob: Vec2::new(1.0, 2.5).normalize() * 1.1,
            roll: Vec2::new(0.4, -0.1),
            spin: 0.0,
        }
    }

    /// [`Self::base`] with the throw values multiplied by `strength`.
    pub fn strength(strength: f32) -> Self {
        let base = Self::base();
        Self {
            normal: base.normal * strength,
            fast: base.fast * strength,
            up: base.up * strength,
            drop: base.drop * strength,
            lob: base.lob * strength,
            roll: base.roll * strength,
            spin: 0.0,
        }
    }

    pub fn with_spin(self, spin: f32) -> Self {
        Self { spin, ..self }
    }

    /// Chooses one of the throw values based on a [`PlayerControl`]
    pub fn velocity_from_control(&self, player_control: &PlayerControl) -> Vec2 {
        let PlayerControl { move_direction, .. } = player_control;
        let y = move_direction.y;
        let moving = move_direction.x.abs() > 0.0;
        if y < 0.0 {
            if moving {
                return self.roll;
            } else {
                return self.drop;
            }
        }
        if moving {
            if y > 0.0 {
                self.lob
            } else {
                self.fast
            }
        } else if y > 0.0 {
            self.up
        } else {
            self.normal
        }
    }
}

pub fn throw_dropped_items(
    entities: Res<Entities>,
    item_throws: Comp<ItemThrow>,
    items: Comp<Item>,
    player_inputs: Res<PlayerInputs>,
    player_indexes: Comp<PlayerIdx>,
    mut items_dropped: CompMut<ItemDropped>,
    mut bodies: CompMut<KinematicBody>,
    mut attachments: CompMut<PlayerBodyAttachment>,
    mut sprites: CompMut<AtlasSprite>,
    mut commands: Commands,
) {
    for (entity, (_items, item_throw, body)) in
        entities.iter_with((&items, &item_throws, &mut bodies))
    {
        if let Some(ItemDropped { player }) = items_dropped.remove(entity) {
            commands.add(PlayerCommand::set_inventory(player, None));
            attachments.remove(entity);

            let player_sprite = sprites.get_mut(player).unwrap();

            let horizontal_flip_factor = if player_sprite.flip_x {
                Vec2::new(-1.0, 1.0)
            } else {
                Vec2::ONE
            };

            let throw_velocity = item_throw.velocity_from_control(
                &player_inputs
                    .players
                    .get(player_indexes.get(player).unwrap().0)
                    .unwrap()
                    .control,
            );

            body.velocity = throw_velocity * horizontal_flip_factor;
            body.angular_velocity =
                item_throw.spin * horizontal_flip_factor.x * throw_velocity.y.signum();

            body.is_deactivated = false;
        }
    }
}
