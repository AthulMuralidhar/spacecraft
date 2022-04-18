#![allow(dead_code)]
#![allow(unused_imports)]

// mod craft;
// mod constants;
// mod window;

// // use snake::{
// //     spawn_snake, snake_movement,snake_movement_input, snake_eating, snake_growth,
// //     SnakeMovement, SnakeSegments, GrowthEvent, LastTailPosition,
// // };
use bevy::app::App;
// use window::{position_translation, size_scaling};
// // use bevy::render::pass::ClearColor;
// use constants::Materials;
// use bevy::core::FixedTimestep;


// https://bevy-cheatbook.github.io/programming/app-builder.html
fn main() {
    App::default()
}
//         .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
//         .insert_resource(WindowDescriptor {
//             title: "Snake!".to_string(),
//             width: 500.0,
//             height: 500.0,
//             ..Default::default()
//         })
//         // .insert_resource(SnakeSegments::default())
//         // .insert_resource(LastTailPosition::default())
//         // .add_event::<GrowthEvent>()
//         .add_startup_system(setup.system())
//         // .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
//         // .add_system(
//         //     snake_movement_input
//         //         .system()
//         //         .label(SnakeMovement::Input)
//         //         .before(SnakeMovement::Movement),
//         // )
//         // .add_system_set(
//         //     SystemSet::new()
//         //         .with_run_criteria(FixedTimestep::step(0.150))
//         //         .with_system(snake_movement.system().label(SnakeMovement::Movement))
//         //         .with_system(
//         //             snake_eating
//         //                 .system()
//         //                 .label(SnakeMovement::Eating)
//         //                 .after(SnakeMovement::Movement),
//         //         )
//         //         .with_system(
//         //             snake_growth
//         //                 .system()
//         //                 .label(SnakeMovement::Growth)
//         //                 .after(SnakeMovement::Eating),
//         //         ),
//         // )
//         .add_system_set_to_stage(
//             CoreStage::PostUpdate,
//             SystemSet::new()
//                 .with_system(position_translation.system())
//                 .with_system(size_scaling.system()),
//         )
//         .add_plugins(DefaultPlugins)
//         .run();
// }


// fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//     commands.insert_resource(Materials {
//         head_material: materials.add(Color::rgb(0.7,0.7,0.7).into()),
//         food_material: materials.add(Color::rgb(1.0,1.0,1.0).into()),
//         segment_material: materials.add(Color::rgb(0.3,0.3,0.3).into())
//     });
// }