use bevy::{
    ecs::Mut,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: String::from("glens game"),
            ..Default::default()
        })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(player_movement_system.system())
        .run();
}

#[derive(Debug)]
struct Player {
    speed: f32,
}

fn setup (
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // spawn player
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Player { speed: 500.0 });
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
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
        // *translation.y_mut() += time.delta_seconds * x_direction * player.speed;
        // *translation.x_mut() = translation.x().min(380.0).max(-380.0);
    }
}

// fn paddle_movement_system(
//     time: Res<Time>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&Paddle, &mut Transform)>,
// ) {
//     for (paddle, mut transform) in &mut query.iter() {
//         let mut direction = 0.0;
//         if keyboard_input.pressed(KeyCode::Left) {
//             direction -= 1.0;
//         }

//         if keyboard_input.pressed(KeyCode::Right) {
//             direction += 1.0;
//         }

//         let translation = transform.translation_mut();
//         // move the paddle horizontally
//         *translation.x_mut() += time.delta_seconds * direction * paddle.speed;
//         // bound the paddle within the walls
//         *translation.x_mut() = translation.x().min(380.0).max(-380.0);
//     }
// }
