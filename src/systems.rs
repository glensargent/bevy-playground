use bevy::{
    // ecs::Mut,
    prelude::*,
    // render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

use crate::components;

#[derive(Debug)]
pub enum Collider {
    Solid,
}

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
        .with(components::Player { velocity: Vec3::new(0.0, 0.0, 0.0), speed: 500.0 })
        .with(Collider::Solid)
        // spawn a basic enemy
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(255.0, 0.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(components::Enemy{ name: "deniz".to_string() })
        .with(Collider::Solid);
}

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut components::Player, &mut Transform)>,
) {
    for (mut player, mut transform) in &mut query.iter() {
        // // let translation = transform.translation_mut();
        if keyboard_input.pressed(KeyCode::A) {
            *player.velocity.x_mut() = -player.speed;
            transform.translate(player.velocity * time.delta_seconds);
            *player.velocity.x_mut() = 0.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            *player.velocity.x_mut() = player.speed;
            transform.translate(player.velocity * time.delta_seconds);
            *player.velocity.x_mut() = 0.0;
        }

        if keyboard_input.pressed(KeyCode::W) {
            *player.velocity.y_mut() = player.speed;
            transform.translate(player.velocity * time.delta_seconds);
            *player.velocity.y_mut() = 0.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            *player.velocity.y_mut() = -player.speed;
            transform.translate(player.velocity * time.delta_seconds);
            *player.velocity.y_mut() = 0.0;
        }
    }
}

pub fn player_collision(
    mut commands: Commands,
    mut player_query: Query<(&mut components::Player, &Transform, &Sprite)>,
    mut collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {

    for (player, mut player_transform, player_sprite) in &mut player_query.iter() {
        // check collision with enemies
        for (collider_entity, _collider, transform, sprite) in &mut collider_query.iter() {
            // collider entity is the thing the player actually touched
            let collision = collide(
                player_transform.translation(), 
                player_sprite.size, 
                transform.translation(), 
                sprite.size
            );

            // will do this for all collision types by default since there's no
            // match for the collider type
            if let Some(collision) = collision {
                // commands.despawn(collider_entity);
                match collision {
                    Collision::Right => {
                        // let test = player_transform.translation_mut();
                        println!("{:?}", player_transform);
                    },
                    Collision::Left => {
                        // let translation = player_transform.translation_mut();
                        // *translation.x_mut() -= 1.0;
                    },
                    Collision::Top => {
                        // let translation = player_transform.translation_mut();
                        // *translation.x_mut() -= 1.0;
                    },
                    Collision::Bottom => {
                        // let translation = player_transform.translation_mut();
                        // *translation.x_mut() -= 1.0;
                    },
                }
            }

        }
    }

}