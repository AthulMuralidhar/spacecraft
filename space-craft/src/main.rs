#[allow(unused_imports)]

use bevy::prelude::*;
// use bevy::render::pass::ClearColor;

// https://bevy-cheatbook.github.io/programming/app-builder.html

fn main() {
    App::new()
        // bevy
        .add_plugins(DefaultPlugins)

        // resources:
        .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
        .insert_resource(WindowDescriptor {
            title: "craft".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        // if it implements `Default` or `FromWorld`
        // .init_resource::<MyFancyResource>()

        // // events:
        // .add_event::<LevelUpEvent>()

        // // systems to run once at startup:
        // .add_startup_system(spawn_player)

        // // systems to run each frame:
        // .add_system(player_level_up)
        // .add_system(debug_levelups)
        // .add_system(debug_stats_change)
        // ...

        // launch the app!
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7,0.7,0.7).into()),
        food_material: materials.add(Color::rgb(1.0,1.0,1.0).into()),
        segment_material: materials.add(Color::rgb(0.3,0.3,0.3).into())
    });
}
