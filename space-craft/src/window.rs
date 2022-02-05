use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use bevy::prelude::*;

#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}


pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

pub fn size_scaling(windows: Res<Windows>, mut q:  Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();

    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        )
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let title_size = bound_window / bound_game;
        pos/bound_game * bound_window - (bound_window/2.0) + (title_size/2.0)
    }

    let window = windows.get_primary().unwrap();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        )
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Home,
}

impl Direction {
    pub fn opposite(self)-> Self {
        match self {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Up=> Self::Down,
            Self::Home => Self::Home,
        }
    }
}

pub struct GameOverEvent;