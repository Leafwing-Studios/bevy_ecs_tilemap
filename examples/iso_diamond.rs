use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
mod helpers;

// This example demonstrates a tilemap laid out isometrically using the "Diamond" coordinate system.

// Side length of a colored quadrant (in "number of tiles").
const QUADRANT_SIDE_LENGTH: u32 = 80;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("iso_color.png");

    // In total, there will be `(QUADRANT_SIDE_LENGTH * 2) * (QUADRANT_SIDE_LENGTH * 2)` tiles.
    let total_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH * 2,
        y: QUADRANT_SIDE_LENGTH * 2,
    };
    let quadrant_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH,
        y: QUADRANT_SIDE_LENGTH,
    };
    let mut tile_storage = TileStorage::empty(total_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_tilemap_rect(
        TileTextureIndex(0),
        TilePos { x: 0, y: 0 },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect(
        TileTextureIndex(1),
        TilePos {
            x: QUADRANT_SIDE_LENGTH,
            y: 0,
        },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect(
        TileTextureIndex(2),
        TilePos {
            x: 0,
            y: QUADRANT_SIDE_LENGTH,
        },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect(
        TileTextureIndex(3),
        TilePos {
            x: QUADRANT_SIDE_LENGTH,
            y: QUADRANT_SIDE_LENGTH,
        },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 64.0, y: 32.0 };
    let grid_size = tile_size.into();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: total_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type: TilemapType::Isometric(IsoCoordSystem::Diamond),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1270.0,
                        height: 720.0,
                        title: String::from("Iso Diamond Example"),
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(helpers::camera::movement)
        .run();
}
