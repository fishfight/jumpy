use macroquad::{
    color,
    experimental::{
        animation::{AnimatedSprite, Animation},
        collections::storage,
        scene::RefMut,
    },
    prelude::*,
};

use crate::Resources;

use super::player::{PLAYER_HITBOX_HEIGHT, PLAYER_HITBOX_WIDTH};

const FLYING_CURSE_COUNTDOWN_DURATION: f32 = 10.;

const FLYING_CURSE_WIDTH: f32 = 32.;
pub const FLYING_CURSE_HEIGHT: f32 = 32.;
const FLYING_CURSE_ANIMATION_FLYING: &'static str = "flying";
const FLYING_CURSE_SPEED: f32 = 70.;
const FLYING_CURSE_MAX_AMPLITUDE: f32 = 100.;
const FLYING_CURSE_Y_FREQ_SLOWDOWN: f32 = 10.; // the higher, the slower the frequency is

/// The FlyingCurse doesn't have a body, as it has a non-conventional (sinuisodal) motion.
pub struct FlyingCurse {
    flying_curse_sprite: AnimatedSprite,
    reference_pos: Vec2,
    current_x: f32,
    current_base_y: f32,
    speed: Vec2,
    facing: bool,
    lived: f32,
    countdown: f32,
    owner_id: u8,
}

impl FlyingCurse {
    pub fn new(curse_pos: Vec2, speed: Vec2, owner_id: u8) -> Self {
        // This can be easily turned into a single sprite, rotated via DrawTextureParams.
        //
        let flying_curse_sprite = AnimatedSprite::new(
            FLYING_CURSE_WIDTH as u32,
            FLYING_CURSE_HEIGHT as u32,
            &[Animation {
                name: FLYING_CURSE_ANIMATION_FLYING.to_string(),
                row: 0,
                frames: 1,
                fps: 1,
            }],
            true,
        );

        let facing = speed.x >= 0.;

        Self {
            flying_curse_sprite,
            current_x: curse_pos.x,
            current_base_y: curse_pos.y,
            reference_pos: vec2(curse_pos.x, curse_pos.y),
            speed,
            facing,
            lived: 0.0,
            countdown: FLYING_CURSE_COUNTDOWN_DURATION,
            owner_id,
        }
    }

    fn current_y(&self) -> f32 {
        // Start always from the negative value, so that the motion always starts upwards.
        let current_pos = vec2(self.current_x, self.current_base_y);
        let distance_from_reference = -(current_pos - self.reference_pos).length().abs();

        let displacement = (distance_from_reference / FLYING_CURSE_Y_FREQ_SLOWDOWN).sin()
            * (FLYING_CURSE_MAX_AMPLITUDE / 2.);
        self.current_base_y + displacement
    }
}

pub struct FlyingCurses {
    flying_curses: Vec<FlyingCurse>,
}

impl FlyingCurses {
    pub fn new() -> Self {
        FlyingCurses {
            flying_curses: vec![],
        }
    }

    /// Spawn a curse, and set the direction towards the closest (in radius) enemy.
    /// If there are no enemies, shoot straight.
    pub fn spawn_flying_curse(&mut self, curse_pos: Vec2, default_facing: bool, owner_id: u8) {
        let distance = Self::find_closest_enemy_distance(curse_pos, default_facing, owner_id);
        let speed = distance.normalize() * FLYING_CURSE_SPEED;

        self.flying_curses
            .push(FlyingCurse::new(curse_pos, speed, owner_id));
    }

    fn find_closest_enemy_distance(curse_pos: Vec2, default_facing: bool, owner_id: u8) -> Vec2 {
        let players = scene::find_nodes_by_type::<crate::nodes::Player>();

        let enemies_pos = players
            .filter_map(|player| {
                if player.id == owner_id {
                    None
                } else {
                    Some(player.body.pos)
                }
            })
            .collect::<Vec<_>>();

        if enemies_pos.len() == 0 {
            let facing_x_factor = if default_facing { 1. } else { -1. };
            return vec2(FLYING_CURSE_SPEED * facing_x_factor, 0.);
        }

        let mut closest_pos = enemies_pos[0];

        for enemy_pos in enemies_pos.into_iter().skip(1) {
            if (curse_pos - enemy_pos).abs() < closest_pos {
                closest_pos = enemy_pos;
            }
        }

        closest_pos - curse_pos
    }
}

impl scene::Node for FlyingCurses {
    fn fixed_update(mut node: RefMut<Self>) {
        for flying_curse in &mut node.flying_curses {
            flying_curse.lived += get_frame_time();
            flying_curse.current_x += flying_curse.speed.x * get_frame_time();
            flying_curse.current_base_y += flying_curse.speed.y * get_frame_time();
        }

        node.flying_curses.retain(|flying_curse| {
            let hit_fxses = &mut storage::get_mut::<Resources>().hit_fxses;
            let explosion_position = vec2(
                flying_curse.current_x + FLYING_CURSE_WIDTH / 2.,
                flying_curse.current_y() + FLYING_CURSE_HEIGHT / 2.,
            );

            if flying_curse.lived < flying_curse.countdown {
                let flying_curse_hitbox = Rect::new(
                    flying_curse.current_x,
                    flying_curse.current_y(),
                    FLYING_CURSE_WIDTH,
                    FLYING_CURSE_HEIGHT,
                );

                for mut player in scene::find_nodes_by_type::<crate::nodes::Player>() {
                    if flying_curse.owner_id != player.id {
                        let player_hitbox = Rect::new(
                            player.body.pos.x,
                            player.body.pos.y,
                            PLAYER_HITBOX_WIDTH,
                            PLAYER_HITBOX_HEIGHT,
                        );
                        if player_hitbox.intersect(flying_curse_hitbox).is_some() {
                            hit_fxses.spawn(explosion_position);

                            scene::find_node_by_type::<crate::nodes::Camera>()
                                .unwrap()
                                .shake();

                            let direction = flying_curse.current_x
                                > (player.body.pos.x + PLAYER_HITBOX_WIDTH / 2.);
                            player.kill(direction);

                            return false;
                        }
                    }
                }

                return true;
            }

            hit_fxses.spawn(explosion_position);

            false
        });
    }

    fn draw(mut node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();
        for flying_curse in &mut node.flying_curses {
            flying_curse.flying_curse_sprite.update();

            draw_texture_ex(
                resources.flying_curses,
                flying_curse.current_x,
                flying_curse.current_y(),
                color::WHITE,
                DrawTextureParams {
                    source: Some(flying_curse.flying_curse_sprite.frame().source_rect),
                    dest_size: Some(flying_curse.flying_curse_sprite.frame().dest_size),
                    flip_x: flying_curse.facing,
                    rotation: 0.0,
                    ..Default::default()
                },
            );
        }
    }
}
