use bevy::math::IVec2;
use bevy::prelude::Component;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct ChunkPos(pub IVec2);
