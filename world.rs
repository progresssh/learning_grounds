use crate::player::Player;
use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};
use bevy_ecs_tilemap::prelude::*;

const TILE_SIZE: f32 = 8.0;
const CHUNK_SIZE: u32 = 32;
const RENDER_DISTANCE: i32 = 3;

#[derive(Component)]
pub struct TerrainChunk {
    pub coord: IVec2,
}

#[derive(Resource, Default)]
pub struct LoadedChunks {
    pub chunks: HashSet<IVec2>,
}

#[derive(Resource)]
pub struct TerrainConfig {
    pub tileset: Handle<Image>,
    pub snow_tile_index: u32,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
            .add_systems(Startup, (spawn_player_chunk, setup_terrain))
            .init_resource::<LoadedChunks>()
            .add_systems(
                Update,
                (
                    update_loaded_chunks,
                    spawn_terrain_chunks,
                    despawn_terrain_chunks,
                )
                    .chain(),
            );
    }
}

fn setup_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tileset_handle = asset_server.load("Tilesheet.png");

    commands.insert_resource(
        (TerrainConfig {
            tileset: tileset_handle,
            snow_tile_index: 20,
        }),
    );
}

fn world_to_chunk_coord(world_pos: Vec2) -> IVec2 {
    let chunk_world_size = TILE_SIZE * CHUNK_SIZE as f32;
    IVec2::new(
        (world_pos.x / chunk_world_size).floor() as i32,
        (world_pos.y / chunk_world_size).floor() as i32,
    )
}

fn chunk_to_world_coor(chunk_coord: IVec2) -> Vec2 {
    let chunk_world_size = TILE_SIZE * CHUNK_SIZE as f32;
    Vec2::new(
        chunk_coord.x as f32 * chunk_world_size,
        chunk_coord.y as f32 * chunk_world_size,
    )
}

fn spawn_player_chunk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("chunk1.ldtk").into(),
        ..Default::default()
    });
}

fn update_loaded_chunks(
    player_query: Query<&Transform, With<Player>>,
    mut loaded_chunks: ResMut<LoadedChunks>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let player_chunk = world_to_chunk_coord(player_pos);

    let mut should_be_loaded = HashSet::new();
    for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for y in -RENDER_DISTANCE..=RENDER_DISTANCE {
            should_be_loaded.insert(player_chunk + IVec2::new(x, y));
        }
    }

    loaded_chunks.chunks = should_be_loaded;
}

fn spawn_terrain_chunks(
    loaded_chunks: Res<LoadedChunks>,
    existing_chunks: Query<&TerrainChunk>,
    terrain_config: Res<TerrainConfig>,
    mut commands: Commands,
) {
    let existing: HashSet<IVec2> = existing_chunks.iter().map(|c| c.coord).collect();

    for &chunk_coord in &loaded_chunks.chunks {
        if existing.contains(&chunk_coord) {
            continue;
        }

        spawn_chunk(&mut commands, &terrain_config, chunk_coord);
    }
}

fn despawn_terrain_chunks(
    loaded_chunks: Res<LoadedChunks>,
    chunks_query: Query<(Entity, &TerrainChunk)>,
    mut commands: Commands,
) {
    for (entity, chunk) in &chunks_query {
        if !loaded_chunks.chunks.contains(&chunk.coord) {
            commands.entity(entity).despawn_children();
        }
    }
}

fn spawn_chunk(commands: &mut Commands, config: &TerrainConfig, chunk_coord: IVec2) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TilemapSize {
        x: CHUNK_SIZE,
        y: CHUNK_SIZE,
    });

    let chunk_world_pos = chunk_to_world_coor(chunk_coord);

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let tile_pos = TilePos { x, y };

            // TODO: Noise-based tile determination
            let tile_index = config.snow_tile_index;

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile_index),
                    ..default()
                })
                .id();

            tile_storage.set(&tile_pos, tile_entity)
        }
    }

    let tile_size = TilemapTileSize {
        x: TILE_SIZE,
        y: TILE_SIZE,
    };

    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: TilemapSize {
                x: CHUNK_SIZE,
                y: CHUNK_SIZE,
            },
            storage: tile_storage,
            texture: TilemapTexture::Single(config.tileset.clone()),
            tile_size,
            transform: Transform::from_translation(chunk_world_pos.extend(0.0)),
            ..default()
        },
        TerrainChunk { coord: chunk_coord },
    ));
}
