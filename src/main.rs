use bevy::{
    // ecs::Mut,
    prelude::*,
    // render::pass::ClearColor,
    // sprite::collide_aabb::{collide, Collision},
};

mod systems;
mod components;
mod resources;

// todo
// make an event system with a collision event
// let the player movement system handle the collision movement

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: String::from("glens game"),
            width: 800,
            height: 720,
            resizable: true,
            vsync: true,
            mode: bevy::window::WindowMode::Windowed,
            ..Default::default()
        })
        .add_default_plugins()
        .add_startup_system(systems::setup.system())
        .add_event::<resources::CollisionEvent>()
        .add_system(systems::player_movement.system())
        .add_system(systems::player_collision.system())
        .run();
}