use bevy::prelude::*;
// use crate::window::{Position, Size, Direction};
// use crate::constants::Materials;

pub struct Head {
    direction: Direction,
}




pub fn spawn_head(
    mut commands: Commands, 
    materials: Res<Materials>,
) {
        commands.spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0,10.0)),
            ..Default::default()
        })
        .insert(Head {
            direction: Direction::Home,
        })
        .insert(Position {x: 3, y:3})
        .insert(Size::square(0.8))
        .id(),
}

pub fn head_movement(
    mut head: Query<(Entity, &Head)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let mut head_pos = positions.get_mut(head_entity).unwrap();

        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
            Direction::Home => {
                head_pos.x = 0;
                head_pos.y = 0;
            }
        };
    }

}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum HeadMovement {
    Input,
    Movement,
}

pub fn head_movement_input(
    keybord_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut HeadHead>
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keybord_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keybord_input.pressed(KeyCode::Down) {
            Direction::Down
        }else if keybord_input.pressed(KeyCode::Up) {
            Direction::Up
        }else if keybord_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}