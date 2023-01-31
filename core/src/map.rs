use std::{
    cmp::{max, min},
    collections::VecDeque,
};

use ::bevy::utils::HashSet;

use crate::prelude::{collisions::TileCollisionKind, *};

pub fn install(session: &mut GameSession) {
    session
        .stages
        .add_system_to_stage(CoreStage::First, spawn_map)
        .add_system_to_stage(CoreStage::First, handle_out_of_bounds_players);
}

/// Resource containing the map metadata for this game session.
#[derive(Clone, TypeUlid, Deref, DerefMut, Default)]
#[ulid = "01GP2H6K9H3JEEMXFCKV4TGMWZ"]
pub struct LoadedMap(pub Arc<MapMeta>);

/// Resource indicating whether the map has been spawned.
#[derive(Clone, TypeUlid, Default, Deref, DerefMut)]
#[ulid = "01GP3Z38HKE37JB6GRHHPPTY38"]
pub struct MapSpawned(pub bool);

/// Helper for getting the z-depth of the map layer with the given index.
pub fn z_depth_for_map_layer(layer_idx: usize) -> f32 {
    // We start map layers at -900 and for ever layer we place a gap of 2 units in between
    -900.0 + layer_idx as f32 * 2.0
}

/// Resource containing essential the map metadata for the map once spawned. This allows the
/// complete map metadata to be re-constructed from the world after the map has been spawned and
/// potentially modified.
#[derive(TypeUlid, Clone)]
#[ulid = "01GSR8V683B3EH5QAB2PMGN9J7"]
pub struct SpawnedMapMeta {
    pub name: Arc<str>,
    pub background: Arc<BackgroundMeta>,
    pub background_color: ColorMeta,
    pub grid_size: UVec2,
    pub tile_size: Vec2,
    pub layer_names: Arc<[String]>,
}

impl Default for SpawnedMapMeta {
    fn default() -> Self {
        Self {
            name: "".into(),
            background: default(),
            background_color: default(),
            grid_size: default(),
            tile_size: default(),
            layer_names: Arc::new([]),
        }
    }
}

/// Component containing the map layer that an entity is associated to.
///
/// This is used when exporting the world to `MapMeta` to decide which layer to put an element or
/// tile layer in.
#[derive(TypeUlid, Clone, Copy, Default)]
#[ulid = "01GSR8GSRJHGTJ8J9Y38W7C5S3"]
pub struct SpawnedMapLayerMeta {
    /// The layer index of the layer that the element belongs to in the map.
    pub layer_idx: usize,
}

/// The map navigation graph resource.
#[derive(Clone, Debug, Deref, DerefMut, TypeUlid, Default)]
#[ulid = "01GQWP4QG11NBVX3M289TXAK6W"]
pub struct NavGraph(pub Option<Arc<NavGraphInner>>);

/// The inner graph type of [`NavGraph`].
pub type NavGraphInner = petgraph::graphmap::DiGraphMap<NavNode, NavGraphEdge>;

/// The type of nodes in the map navigation graph.
///
/// This is merely a wrapper around [`UVec2`] to add an [`Ord`] implementation.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Deref, DerefMut)]
pub struct NavNode(pub UVec2);

impl NavNode {
    /// Calculates the Pythagorean distance between two nodes.
    pub fn distance(&self, other: &Self) -> f32 {
        let dx = (max(self.x, other.x) - min(self.x, other.x)) as f32;
        let dy = (max(self.y, other.y) - min(self.y, other.y)) as f32;
        (dx * dx) + (dy * dy)
    }

    pub fn right(&self) -> NavNode {
        NavNode(self.0 + uvec2(1, 0))
    }

    pub fn above(&self) -> NavNode {
        NavNode(self.0 + uvec2(0, 1))
    }

    pub fn left(&self) -> Option<NavNode> {
        if self.0.x > 0 {
            Some(NavNode(self.0 - uvec2(1, 0)))
        } else {
            None
        }
    }

    pub fn below(&self) -> Option<NavNode> {
        if self.0.y > 0 {
            Some(NavNode(self.0 - uvec2(0, 1)))
        } else {
            None
        }
    }

    pub fn below_left(&self) -> Option<NavNode> {
        self.left().and_then(|x| x.below())
    }

    pub fn below_right(&self) -> Option<NavNode> {
        self.below().map(|x| x.right())
    }

    pub fn above_left(&self) -> Option<NavNode> {
        self.left().map(|x| x.above())
    }

    pub fn above_right(&self) -> NavNode {
        self.right().above()
    }
}
impl From<UVec2> for NavNode {
    fn from(v: UVec2) -> Self {
        Self(v)
    }
}
impl From<NavNode> for UVec2 {
    fn from(v: NavNode) -> Self {
        v.0
    }
}
impl std::cmp::Ord for NavNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl std::cmp::PartialOrd for NavNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let xcmp = self.0.x.cmp(&other.0.x);
        Some(if xcmp == std::cmp::Ordering::Equal {
            self.0.y.cmp(&other.0.y)
        } else {
            xcmp
        })
    }
}

/// Represents the way to get from one tile to another tile in the navigation graph.
#[derive(Debug, Clone)]
pub struct NavGraphEdge {
    /// The sequence of inputs for each frame, required to get to the connected tile.
    pub inputs: VecDeque<PlayerControl>,
    /// The distance to the connected tile. This is used as the heuristic for pathfinding.
    pub distance: f32,
}

fn spawn_map(
    mut commands: Commands,
    mut entities: ResMut<Entities>,
    mut clear_color: ResMut<ClearColor>,
    map: Res<LoadedMap>,
    mut map_spawned: ResMut<MapSpawned>,
    mut tiles: CompMut<Tile>,
    mut tile_layers: CompMut<TileLayer>,
    mut transforms: CompMut<Transform>,
    mut element_handles: CompMut<ElementHandle>,
    mut tile_collisions: CompMut<TileCollisionKind>,
    mut parallax_bg_sprites: CompMut<ParallaxBackgroundSprite>,
    mut sprites: CompMut<Sprite>,
    mut nav_graph: ResMut<NavGraph>,
    mut cameras: CompMut<Camera>,
    mut camera_shakes: CompMut<CameraShake>,
    mut camera_states: CompMut<CameraState>,
    mut spawned_map_layer_metas: CompMut<SpawnedMapLayerMeta>,
    mut spawned_map_meta: ResMut<SpawnedMapMeta>,
) {
    if map_spawned.0 {
        return;
    }

    // Fill in the spawned map metadata
    *spawned_map_meta = SpawnedMapMeta {
        name: map.name.clone().into(),
        background: Arc::new(map.background.clone()),
        background_color: map.background_color,
        grid_size: map.grid_size,
        tile_size: map.tile_size,
        layer_names: map.layers.iter().map(|x| x.id.to_string()).collect(),
    };

    // Spawn the camera
    {
        let ent = entities.create();
        camera_shakes.insert(
            ent,
            CameraShake {
                center: (map.tile_size * (map.grid_size / 2).as_vec2()).extend(0.0),
                ..CameraShake::new(2.0, glam::vec2(20.0, 20.0), 1.0, 1.0)
            },
        );
        cameras.insert(ent, default());
        transforms.insert(ent, default());
        camera_states.insert(ent, default());
    }

    map_spawned.0 = true;
    **clear_color = map.background_color.0;

    // Load the navigation graph
    nav_graph.0 = Some(create_nav_graph(&map));

    // Spawn parallax backgrounds
    for layer in &map.background.layers {
        for i in -1..=1 {
            let ent = entities.create();
            sprites.insert(
                ent,
                Sprite {
                    image: layer.image.clone(),
                    ..default()
                },
            );
            transforms.insert(ent, default());
            parallax_bg_sprites.insert(
                ent,
                ParallaxBackgroundSprite {
                    idx: i,
                    meta: layer.clone(),
                },
            );
        }
    }

    // Load tiles
    for (layer_idx, layer) in map.layers.iter().enumerate() {
        let layer_z = z_depth_for_map_layer(layer_idx);
        let mut tile_layer = TileLayer::new(
            map.grid_size,
            map.tile_size,
            // Just use a dummy atlas if one is not specified
            layer.tilemap.clone().unwrap_or_default(),
        );

        for tile_meta in &layer.tiles {
            let tile_ent = entities.create();
            tile_layer.set(tile_meta.pos, Some(tile_ent));
            tiles.insert(
                tile_ent,
                Tile {
                    idx: tile_meta.idx as usize,
                    ..default()
                },
            );
            tile_collisions.insert(tile_ent, tile_meta.collision);
        }
        let layer_ent = entities.create();
        spawned_map_layer_metas.insert(layer_ent, SpawnedMapLayerMeta { layer_idx });
        tile_layers.insert(layer_ent, tile_layer);
        transforms.insert(
            layer_ent,
            Transform::from_translation(Vec3::new(0.0, 0.0, layer_z)),
        );

        for element_meta in &layer.elements {
            let element_ent = entities.create();

            spawned_map_layer_metas.insert(element_ent, SpawnedMapLayerMeta { layer_idx });
            transforms.insert(
                element_ent,
                Transform::from_translation(element_meta.pos.extend(layer_z)),
            );
            element_handles.insert(element_ent, ElementHandle(element_meta.element.clone()));
        }
    }

    // Update collision world with map tiles
    commands.add(|mut collision_world: CollisionWorld| {
        collision_world.update_tiles();
    });
}

fn handle_out_of_bounds_players(
    entities: Res<Entities>,
    mut commands: Commands,
    transforms: CompMut<Transform>,
    player_indexes: Comp<PlayerIdx>,
    map: Res<LoadedMap>,
) {
    for (player_ent, (_player_idx, transform)) in entities.iter_with((&player_indexes, &transforms))
    {
        if map.is_out_of_bounds(&transform.translation) {
            commands.add(PlayerCommand::kill(player_ent, None));
        }
    }
}

/// Helper method to create a navigation graph from the map metadata.
fn create_nav_graph(meta: &MapMeta) -> Arc<NavGraphInner> {
    // Load the navigation graph
    let mut graph = NavGraphInner::default();

    // Initialize set of traversable tiles, assuming all tiles are traversable
    let mut semi_solids = HashSet::default();
    for x in 0..meta.grid_size.x {
        for y in 0..meta.grid_size.y {
            graph.add_node(NavNode(uvec2(x, y)));
        }
    }
    // Find all solid tiles and remove them from the traversable tiles list
    for layer in &meta.layers {
        for tile in &layer.tiles {
            if tile.collision == TileCollisionKind::JumpThrough {
                semi_solids.insert(NavNode(tile.pos));
            } else {
                graph.remove_node(NavNode(tile.pos));
            }
        }
    }

    // Calculate possible movements from every node
    macro_rules! is_solid {
        ($node:ident) => {
            !graph.contains_node($node) || semi_solids.contains(&$node)
        };
    }

    for node in graph.nodes().collect::<Vec<_>>() {
        // walk left or right along the ground
        let has_ground = node.below().map(|x| is_solid!(x)).unwrap_or_default()
            || node.below_left().map(|x| is_solid!(x)).unwrap_or_default()
            || node.below_right().map(|x| is_solid!(x)).unwrap_or_default();

        /////////////////
        // Grounded
        /////////////////

        if has_ground {
            // Moving Right
            let right = node.right();
            if graph.contains_node(right) {
                graph.add_edge(
                    node,
                    right,
                    NavGraphEdge {
                        inputs: [PlayerControl {
                            moving: true,
                            move_direction: vec2(1.0, 0.0),
                            ..default()
                        }]
                        .into(),
                        distance: node.distance(&right),
                    },
                );
            }

            // Moving Left
            if let Some(left) = node.left() {
                if graph.contains_node(left) {
                    graph.add_edge(
                        node,
                        left,
                        NavGraphEdge {
                            inputs: [PlayerControl {
                                moving: true,
                                move_direction: vec2(-1.0, 0.0),
                                ..default()
                            }]
                            .into(),
                            distance: node.distance(&left),
                        },
                    );
                }
            }

            /////////////////
            // JUMPING
            /////////////////
            let above1 = node.above();
            let above2 = above1.above();
            let above3 = above2.above();

            // let above_l = above3.left();
            // let above_r = above3.right();

            if graph.contains_node(above1) && graph.contains_node(above2) {
                let contains_above = graph.contains_node(above3);
                // let contains_above_r = graph.contains_node(above_r);
                // let contains_above_l = above_l.map(|x| graph.contains_node(x)).unwrap_or_default();

                if contains_above {
                    // Jump staight up
                    graph.add_edge(
                        node,
                        above3,
                        NavGraphEdge {
                            inputs: [PlayerControl {
                                jump_just_pressed: true,
                                jump_pressed: true,
                                ..default()
                            }]
                            .into(),
                            distance: node.distance(&above3),
                        },
                    );
                }

                // // Jump Right
                // if graph.contains_node(above_r) {
                //     graph.add_edge(
                //         node,
                //         above3,
                //         NavGraphEdge {
                //             inputs: [PlayerControl {
                //                 move_direction: vec2(1.0, 0.0),
                //                 jump_just_pressed: true,
                //                 jump_pressed: true,
                //                 ..default()
                //             }]
                //             .into(),
                //             distance: node.distance(&above_r),
                //         },
                //     );
                // }
                // // Jump up and left
                // else if above_l.map(|x| graph.contains_node(x)).unwrap_or_default() {
                // graph.add_edge(
                //     node,
                //     above3,
                //     NavGraphEdge {
                //         inputs: [PlayerControl {
                //             move_direction: vec2(-1.0, 0.0),
                //             jump_just_pressed: true,
                //             jump_pressed: true,
                //             ..default()
                //         }]
                //         .into(),
                //         distance: node.distance(&above_l.unwrap()),
                //     },
                // );
                // }
            }
        }

        /////////////////
        // Falling Down
        /////////////////

        // Fall straight down
        if let Some(below) = node.below() {
            if graph.contains_node(below) {
                if semi_solids.contains(&below) {
                    graph.add_edge(
                        node,
                        below,
                        NavGraphEdge {
                            inputs: [
                                PlayerControl {
                                    move_direction: vec2(0.0, -1.0),
                                    jump_just_pressed: true,
                                    jump_pressed: true,
                                    ..default()
                                },
                                PlayerControl {
                                    move_direction: vec2(0.0, -1.0),
                                    jump_pressed: true,
                                    ..default()
                                },
                                PlayerControl::default(),
                                PlayerControl::default(),
                            ]
                            .into(),
                            distance: node.distance(&below),
                        },
                    );
                } else {
                    graph.add_edge(
                        node,
                        below,
                        NavGraphEdge {
                            inputs: [PlayerControl::default()].into(),
                            distance: node.distance(&below),
                        },
                    );
                }
            }
        }

        // Fall diagonally down right
        if let Some(below_right) = node.below_right() {
            if graph.contains_node(below_right) {
                if semi_solids.contains(&below_right) {
                    graph.add_edge(
                        node,
                        below_right,
                        NavGraphEdge {
                            inputs: [
                                PlayerControl {
                                    move_direction: vec2(1.0, -1.0),
                                    jump_just_pressed: true,
                                    jump_pressed: true,
                                    ..default()
                                },
                                PlayerControl {
                                    move_direction: vec2(1.0, -1.0),
                                    jump_pressed: true,
                                    ..default()
                                },
                                PlayerControl {
                                    move_direction: vec2(1.0, 0.0),
                                    ..default()
                                },
                                PlayerControl {
                                    move_direction: vec2(1.0, 0.0),
                                    ..default()
                                },
                            ]
                            .into(),
                            distance: node.distance(&below_right),
                        },
                    );
                } else {
                    graph.add_edge(
                        node,
                        below_right,
                        NavGraphEdge {
                            inputs: [PlayerControl {
                                move_direction: vec2(1.0, 0.0),
                                ..default()
                            }]
                            .into(),
                            distance: node.distance(&below_right),
                        },
                    );
                }
            }
        }
        // Fall diagonally down left
        if let Some(left) = node.below_left() {
            if graph.contains_node(left) {
                if semi_solids.contains(&left) {
                    graph.add_edge(
                        node,
                        left,
                        NavGraphEdge {
                            inputs: [
                                PlayerControl {
                                    move_direction: vec2(-1.0, -1.0),
                                    jump_just_pressed: true,
                                    jump_pressed: true,
                                    ..default()
                                },
                                PlayerControl {
                                    move_direction: vec2(-1.0, -1.0),
                                    jump_pressed: true,
                                    ..default()
                                },
                                PlayerControl {
                                    move_direction: vec2(-1.0, 0.0),
                                    ..default()
                                },
                                PlayerControl {
                                    move_direction: vec2(-1.0, 0.0),
                                    ..default()
                                },
                            ]
                            .into(),
                            distance: node.distance(&left),
                        },
                    );
                } else {
                    graph.add_edge(
                        node,
                        left,
                        NavGraphEdge {
                            inputs: [PlayerControl {
                                move_direction: vec2(-1.0, 0.0),
                                ..default()
                            }]
                            .into(),
                            distance: node.distance(&left),
                        },
                    );
                }
            }
        }
    }

    Arc::new(graph)
}
