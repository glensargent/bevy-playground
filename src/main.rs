use bevy::{
    // ecs::Mut,
    prelude::*,
    // render::pass::ClearColor,
    // sprite::collide_aabb::{collide, Collision},
};

mod systems;
mod components;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: String::from("glens game"),
            ..Default::default()
        })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(systems::player_movement_system.system())
        .run();
}

fn setup (
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // spawn player
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(components::Player { speed: 500.0 });
}