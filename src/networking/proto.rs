//! Serializable data types for network messages used by the game.

use numquant::{IntRange, Quantized};

use crate::prelude::*;

bitfield::bitfield! {
    /// A player's controller inputs densely packed into a single u16.
    ///
    /// This is used when sending player inputs across the network.
    #[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, PartialEq, Eq, Reflect)]
    #[repr(transparent)]
    pub struct DensePlayerControl(u16);
    impl Debug;
    pub jump_pressed, set_jump_pressed: 0;
    pub shoot_pressed, set_shoot_pressed: 1;
    pub grab_pressed, set_grab_pressed: 2;
    pub slide_pressed, set_slide_pressed: 3;
    pub from into DenseMoveDirection, move_direction, set_move_direction: 15, 4;
}

impl Default for DensePlayerControl {
    fn default() -> Self {
        let mut control = Self(0);
        control.set_move_direction(default());
        control
    }
}

/// A newtype around [`Vec2`] that implements [`From<u16>`] and [`Into<u16>`] as a way to compress
/// user stick input for use in [`DensePlayerControl`].
#[derive(Debug, Deref, DerefMut, Default)]
pub struct DenseMoveDirection(pub Vec2);

/// This is the specific [`Quantized`] type that we use to represent movement directions in
/// [`DenseMoveDirection`].
type MoveDirQuant = Quantized<IntRange<u16, 0b111111, -1, 1>>;

impl From<u16> for DenseMoveDirection {
    fn from(bits: u16) -> Self {
        // maximum movement value representable, we use 6 bits to represent each movement direction.
        let max = 0b111111;
        // The first six bits represent the x movement
        let x_move_bits = bits & max;
        // The second six bits represents the y movement
        let y_move_bits = (bits >> 6) & max;

        // Round near-zero values to zero
        let mut x = MoveDirQuant::from_raw(x_move_bits).to_f32();
        if x.abs() < 0.02 {
            x = 0.0;
        }
        let mut y = MoveDirQuant::from_raw(y_move_bits).to_f32();
        if y.abs() < 0.02 {
            y = 0.0;
        }

        DenseMoveDirection(Vec2::new(x, y))
    }
}

impl From<DenseMoveDirection> for u16 {
    fn from(dir: DenseMoveDirection) -> Self {
        let x_bits = MoveDirQuant::from_f32(dir.x).raw();
        let y_bits = MoveDirQuant::from_f32(dir.y).raw();

        x_bits | (y_bits << 6)
    }
}
