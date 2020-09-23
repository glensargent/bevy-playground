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
        .add_startup_system(systems::setup.system())
        .add_system(systems::player_movement.system())
        .add_system(systems::player_collision.system())
        .run();
}