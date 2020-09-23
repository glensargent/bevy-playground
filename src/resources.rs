use bevy::{
    // ecs::Mut,
    prelude::*,
    // render::pass::ClearColor,
};

// collision between two entities
// the order of entities doesnt matter when comparing
#[derive(Debug, Clone)]
pub struct CollisionEvent(pub Entity, pub Entity);

impl CollisionEvent {
    pub fn new(e1: Entity, e2: Entity) -> Self {
        CollisionEvent(e1, e2)
    }
}

impl PartialEq for CollisionEvent {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}