use std::any::TypeId;

use macroquad::{prelude::*, ui::Ui};

mod confirm_dialog;
mod create_tileset;

mod create_layer;
mod tileset_properties;

pub use confirm_dialog::ConfirmDialog;
pub use create_layer::CreateLayerWindow;

use super::{ButtonParams, EditorAction, EditorContext, Map};
pub use create_tileset::CreateTilesetWindow;
pub use tileset_properties::TilesetPropertiesWindow;

pub const WINDOW_BUTTON_HEIGHT: f32 = 32.0;

pub const WINDOW_BUTTON_MIN_WIDTH: f32 = 64.0;
pub const WINDOW_BUTTON_MAX_WIDTH: f32 = 96.0;

#[derive(Debug, Copy, Clone)]
pub enum WindowPosition {
    Centered,
    Absolute(Vec2),
}

impl WindowPosition {
    pub fn to_absolute(&self, size: Vec2) -> Vec2 {
        match self {
            WindowPosition::Centered => {
                let screen_size = vec2(screen_width(), screen_height());
                (screen_size - size) / 2.0
            }
            WindowPosition::Absolute(position) => *position,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WindowParams {
    pub title: Option<String>,
    pub size: Vec2,
    pub position: WindowPosition,
    pub has_buttons: bool,
    pub is_static: bool,
}

impl WindowParams {
    pub fn get_absolute_position(&self) -> Vec2 {
        self.position.to_absolute(self.size)
    }
}

impl Default for WindowParams {
    fn default() -> Self {
        WindowParams {
            title: None,
            size: vec2(250.0, 350.0),
            position: WindowPosition::Centered,
            has_buttons: true,
            is_static: false,
        }
    }
}

// This should be implemented for any windows opened by the gui.
// It provides a uniform standardization of window, boot visually and in regards to how control
// flow is handled, by returning actions, in stead of performing the logic in the window draw.
pub trait Window {
    fn get_params(&self) -> &WindowParams;

    // Implement this and set `has_buttons` to true in the `WindowParams` returned by
    // `get_params` to add buttons to the bottom of the window.
    fn get_buttons(&self, _map: &Map, _ctx: &EditorContext) -> Vec<ButtonParams>
    where
        Self: 'static,
    {
        vec![ButtonParams {
            label: "Close",
            action: Some(self.get_close_action()),
            ..Default::default()
        }]
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        size: Vec2,
        map: &Map,
        ctx: &EditorContext,
    ) -> Option<EditorAction>;

    fn get_absolute_position(&self) -> Vec2 {
        let params = self.get_params();
        params.position.to_absolute(params.size)
    }

    fn get_rect(&self) -> Rect {
        let params = self.get_params();
        let position = params.position.to_absolute(params.size);
        Rect::new(position.x, position.y, params.size.x, params.size.y)
    }

    fn contains(&self, point: Vec2) -> bool {
        let rect = self.get_rect();
        rect.contains(point)
    }

    fn get_close_action(&self) -> EditorAction
    where
        Self: 'static,
    {
        let id = TypeId::of::<Self>();
        EditorAction::CloseWindow(id)
    }
}
