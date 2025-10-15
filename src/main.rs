use bevy::prelude::*;

const TILE_SIZE: f32 = 100.0;
const TILE_MARGIN: f32 = 5.0;
const TRANSLATION_OFFSET_X: f32 = (TILE_SIZE * 2.5) + (TILE_MARGIN * 2.0);
const TRANSLATION_OFFSET_Y: f32 = (TILE_SIZE * 2.5) + (TILE_MARGIN * 2.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let tile = Tile {
        letter: Option::None,
        color: Color::WHITE,
    };

    let game_state = GameState {
        tiles: [[tile; 5]; 6],
        current_row: 0,
        current_index: 0,
    };

    for (row_index, row) in game_state.tiles.iter().enumerate() {
        for (index, tile) in row.iter().enumerate() {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(tile.color))),
                Transform {
                    translation: Vec3::new(
                        (index as f32 * (TILE_SIZE + TILE_MARGIN)) - TRANSLATION_OFFSET_X,
                        (row_index as f32 * (TILE_SIZE + TILE_MARGIN)) - TRANSLATION_OFFSET_Y,
                        0.0,
                    ),
                    ..default()
                },
            ));
        }
    }
}

struct GameState {
    tiles: [[Tile; 5]; 6],
    current_row: u8,
    current_index: u8,
}
#[derive(Clone, Copy)]
struct Tile {
    letter: Option<char>,
    color: Color,
}
