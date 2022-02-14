#![allow(unused_imports)]
#![allow(dead_code)]

use bevy::prelude::*;

// https://bevy-cheatbook.github.io/programming/app-builder.html

mod constants;
use constants::Materials;

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
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7,0.7,0.7).into()),
    });
}
