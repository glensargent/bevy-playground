use bevy::{
    // ecs::Mut,
    prelude::*,
    // render::pass::ClearColor,
    // sprite::collide_aabb::{collide, Collision},
};

use crate::components;

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&components::Player, &mut Transform)>,
) {
    for (player, mut transform) in &mut query.iter() {
        let mut x_direction = 0.0;
        let mut y_direction = 0.0;
        let translation = transform.translation_mut();

        if keyboard_input.pressed(KeyCode::A) {
            x_direction -= 1.0;
            *translation.x_mut() += time.delta_seconds * x_direction * player.speed;
        }

        if keyboard_input.pressed(KeyCode::D) {
            x_direction += 1.0;
            *translation.x_mut() += time.delta_seconds * x_direction * player.speed;
        }

        if keyboard_input.pressed(KeyCode::W) {
            y_direction += 1.0;
            *translation.y_mut() += time.delta_seconds * y_direction * player.speed;
        }

        if keyboard_input.pressed(KeyCode::S) {
            y_direction -= 1.0;
            *translation.y_mut() += time.delta_seconds * y_direction * player.speed;
        }

        // handle up and down
        // *translation.x_mut() = translation.x().min(380.0).max(-380.0);
    }
}