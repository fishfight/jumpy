use core::error::ErrorKind;

use core::error::Error;

use std::cmp::Ordering;

use core::error::Result;

use macroquad::prelude::Vec2;

use crate::map::Map;

use super::UndoableAction;

#[derive(Debug)]
pub struct MoveSpawnPoint {
    index: usize,
    position: Vec2,
    old_position: Option<Vec2>,
}

impl MoveSpawnPoint {
    pub fn new(index: usize, position: Vec2) -> Self {
        MoveSpawnPoint {
            index,
            position,
            old_position: None,
        }
    }
}

impl UndoableAction for MoveSpawnPoint {
    fn apply_to(&mut self, map: &mut Map) -> Result<()> {
        let old_position = map.spawn_points.remove(self.index);
        self.old_position = Some(old_position);

        match map.spawn_points.len().cmp(&self.index) {
            Ordering::Equal => map.spawn_points.push(self.position),
            Ordering::Greater => map.spawn_points.insert(self.index, self.position),
            _ => {
                return Err(Error::new_const(
                    ErrorKind::EditorAction,
                    &"MoveSpawnPoint: Index out of bounds",
                ))
            }
        }

        Ok(())
    }

    fn undo(&mut self, map: &mut Map) -> Result<()> {
        if let Some(old_position) = self.old_position {
            map.spawn_points[self.index] = old_position;
        } else {
            return Err(Error::new_const(ErrorKind::EditorAction, &"MoveSpawnPoint (Undo): No old position saved in action. Undo was probably called on an action that was never applied"));
        }

        Ok(())
    }
}
