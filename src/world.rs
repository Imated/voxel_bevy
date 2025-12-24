use bevy::prelude::ReflectResource;
use std::collections::HashMap;
use std::sync::Arc;
use bevy::app::{App, Plugin};
use bevy::math::IVec3;
use bevy::prelude::{Entity, Resource};
use bevy::tasks::Task;
use bevy_inspector_egui::__macro_exports::bevy_reflect::Reflect;
use crate::chunk::Chunk;
use crate::chunk_mesh::ChunkMesh;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

#[derive(Resource, Debug, Default)]
pub struct World {
    pub(crate) chunks: HashMap<IVec3, Arc<Chunk>>,

    pub(crate) chunks_data_to_load: Vec<IVec3>,
    pub(crate) chunks_data_to_unload: Vec<IVec3>,

    pub(crate) chunks_mesh_to_load: Vec<IVec3>,
    pub(crate) chunks_mesh_to_unload: Vec<IVec3>,

    pub(crate) data_tasks: HashMap<IVec3, Task<Chunk>>,
    pub(crate) mesh_tasks: HashMap<IVec3, Task<ChunkMesh>>,

    chunk_entities: HashMap<IVec3, Entity>,
}

impl World {
    pub fn load_chunk(&mut self, position: IVec3, dummy: bool) {
        self.chunks_data_to_load.push(position);
        if !dummy {
            self.chunks_mesh_to_load.push(position);
        }
    }

    pub fn unload_chunk(&mut self, position: IVec3) {
        self.chunks_data_to_unload.push(position);
        self.chunks_mesh_to_unload.push(position);
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(World::default());
    }
}

impl WorldPlugin {

}