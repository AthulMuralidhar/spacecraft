use bevy::prelude::*;

pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;

pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
    pub food_material: Handle<ColorMaterial>,
    pub segment_material:  Handle<ColorMaterial>
}