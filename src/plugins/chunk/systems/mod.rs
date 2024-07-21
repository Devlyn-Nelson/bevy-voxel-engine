use bevy::prelude::*;
use load_system::spawn_gen_task;

use crate::{common::components::pos::PosComponent, plugins::generator::resources::GeneratorRes};

use super::resources::InWorldChunks;

pub mod deform_system;
pub mod load_system;
pub mod loading_control_system;
pub mod unload_system;

#[derive(Resource, Default  )]
pub struct GenerateChunkControlRefresh;

pub fn chunk_generation_control(
    mut commands: Commands,
    mut in_world_chunks: ResMut<InWorldChunks>,
    mut generator: ResMut<GeneratorRes>,
    refresh: Option<Res<GenerateChunkControlRefresh>>,
){
    if refresh.is_some() {
        generator.reroll_seed();
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    let pos = PosComponent::new(x, y, z);
                    commands.remove_resource::<GenerateChunkControlRefresh>();
                    if in_world_chunks.0.contains_key(&pos) {
                        if in_world_chunks.0.remove(&pos).is_none() {
                            warn!("fail to remove refreshing chunk");
                        }
                    }
                    spawn_gen_task(pos, &mut commands, in_world_chunks.as_mut(), generator.as_ref());
                }
            }
        }
    }
}

pub fn chunk_generation_control_start_up(
    mut commands: Commands,
    mut in_world_chunks: ResMut<InWorldChunks>,
    generator: Res<GeneratorRes>,
){
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                let pos = PosComponent::new(x, y, z);
                spawn_gen_task(pos, &mut commands, in_world_chunks.as_mut(), generator.as_ref());
            }
        }
    }
    
}
