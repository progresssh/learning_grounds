use bevy::{ecs::system::command::insert_resource, prelude::*};
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chunk)
        .insert_resource(LevelSelection::index(0));
    }
}

fn spawn_chunk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("chunk1.ldtk").into(),
        ..Default::default()
    }); 
}