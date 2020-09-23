use bevy::{
    prelude::*,
};

#[derive(Debug)]
// pub struct 
pub struct Player {
    pub speed: f32,
    pub velocity: Vec3,
}

pub struct Enemy {
    pub name: String,
}