use bevy::{
    // ecs::Mut,
    prelude::*,
    // render::pass::ClearColor,
    // sprite::collide_aabb::{collide, Collision},
};

use crate::components;

pub fn setup (
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font_handle = asset_server.load("assets/fonts/Roboto-Regular.ttf").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // build ui
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                border: Rect::all(Val::Px(2.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // text
            parent.spawn(TextComponents {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    ..Default::default()
                },
                text: Text {
                    value: "my hotbar:".to_string(),
                    font: font_handle,
                    style: TextStyle {
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                },
                ..Default::default()
            });
        })
        // spawn player
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(components::Player { speed: 500.0 });
}

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