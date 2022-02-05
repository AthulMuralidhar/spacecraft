// use bevy::prelude::*;
// use crate::window::{Position, Size, Direction};
// use crate::constants::Materials;

// pub struct SnakeHead {
//     direction: Direction,
// }
// pub struct GrowthEvent;

// #[derive(Default)]
// pub struct LastTailPosition(Option<Position>);


// pub struct SnakeSegment;

// #[derive(Default)]
// pub struct SnakeSegments(Vec<Entity>);

// pub fn spawn_segment(
//     mut commands: Commands,
//     material: &Handle<ColorMaterial>,
//     position: Position,
    
// )-> Entity {
//     commands.spawn_bundle(SpriteBundle {
//         material: material.clone(),
//         ..Default::default()
//     })
//     .insert(SnakeSegment)
//     .insert(position)
//     .insert(Size::square(0.65))
//     .id()
// }


// pub fn spawn_snake(
//     mut commands: Commands, 
//     materials: Res<Materials>,
//     mut segments: ResMut<SnakeSegments>,
// ) {
//     segments.0 = vec![
//         commands.spawn_bundle(SpriteBundle {
//             material: materials.head_material.clone(),
//             sprite: Sprite::new(Vec2::new(10.0,10.0)),
//             ..Default::default()
//         })
//         .insert(SnakeHead {
//             direction: Direction::Home,
//         })
//         .insert(SnakeSegment)
//         .insert(Position {x: 3, y:3})
//         .insert(Size::square(0.8))
//         .id(),
//         spawn_segment(
//             commands,
//             &materials.segment_material,
//             Position {x: 3, y: 2}
//         ), 
//     ];
// }

// pub fn snake_movement(
//     mut heads: Query<(Entity, &SnakeHead)>,
//     segments: ResMut<SnakeSegments>,
//     mut positions: Query<&mut Position>,
//     mut last_tail_position: ResMut<LastTailPosition>,
// ) {
//     if let Some((head_entity, head)) = heads.iter_mut().next() {
//         let segment_positions = segments.0.iter()
//             .map(|e| *positions.get_mut(*e).unwrap())
//             .collect::<Vec<Position>>();
        
//         let mut head_pos = positions.get_mut(head_entity).unwrap();


//         match &head.direction {
//             Direction::Left => {
//                 head_pos.x -= 1;
//             }
//             Direction::Right => {
//                 head_pos.x += 1;
//             }
//             Direction::Up => {
//                 head_pos.y += 1;
//             }
//             Direction::Down => {
//                 head_pos.y -= 1;
//             }
//             Direction::Home => {
//                 head_pos.x = 0;
//                 head_pos.y = 0;
//             }
//         };
//         segment_positions.iter().zip(
//             segments.0.iter().skip(1)
//         ).for_each(|(pos, segment)| {
//             *positions.get_mut(*segment).unwrap() = *pos;
//         });
    
//         last_tail_position.0 = Some(*segment_positions.last().unwrap())
//     }

// }

// #[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
// pub enum SnakeMovement {
//     Input,
//     Movement,
//     Eating,
//     Growth,
// }

// pub fn snake_movement_input(
//     keybord_input: Res<Input<KeyCode>>,
//     mut heads: Query<&mut SnakeHead>
// ) {
//     if let Some(mut head) = heads.iter_mut().next() {
//         let dir: Direction = if keybord_input.pressed(KeyCode::Left) {
//             Direction::Left
//         } else if keybord_input.pressed(KeyCode::Down) {
//             Direction::Down
//         }else if keybord_input.pressed(KeyCode::Up) {
//             Direction::Up
//         }else if keybord_input.pressed(KeyCode::Right) {
//             Direction::Right
//         } else {
//             head.direction
//         };

//         if dir != head.direction.opposite() {
//             head.direction = dir;
//         }
//     }
// }





// pub fn snake_eating(
//     mut commands: Commands,
//     mut growth_writer: EventWriter<GrowthEvent>,
//     food_positions: Query<(Entity, &Position), With<Food>>,
//     head_positions: Query<&Position, With<SnakeHead>>,
// ) {
//     for head_pos in head_positions.iter() {
//         for (ent, food_pos) in food_positions.iter() {
//             if food_pos == head_pos {
//                 commands.entity(ent).despawn();
//                 growth_writer.send(GrowthEvent);
//             }
//         }
//     }
// }

// pub fn snake_growth(
//     commands: Commands,
//     last_tail_position: Res<LastTailPosition>,
//     mut segments: ResMut<SnakeSegments>,
//     mut growth_reader: EventReader<GrowthEvent>,
//     materials: Res<Materials>    
// ) {
//     if growth_reader.iter().next().is_some() {
//         segments.0.push(spawn_segment(
//             commands,
//             &materials.segment_material,
//             last_tail_position.0.unwrap(),
//         ));
//     }
// }
