use bevy::prelude::*;

const TILE_SIZE: f32 = 100.0;
const TILE_MARGIN: f32 = 5.0;
const TRANSLATION_OFFSET_X: f32 = (TILE_SIZE * 2.5) + (TILE_MARGIN * 2.0);
const TRANSLATION_OFFSET_Y: f32 = -((TILE_SIZE * 3.0) + (TILE_MARGIN * 2.5));

#[derive(Resource)]
struct GameState {
    answer: String,
    current_row: usize,
    current_index: usize,
}

#[derive(Clone, Copy, Component)]
struct Tile {
    letter: Option<char>,
    color: Color,
}

#[derive(Component)]
struct Location {
    x: usize,
    y: usize,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, handle_keyboard_input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.insert_resource(GameState {
        answer: String::from("hello"),
        current_row: 0,
        current_index: 0,
    });

    // build game grid
    let grid: [[usize; 5]; 6] = [[0; 5]; 6];
    for (row_index, row) in grid.iter().enumerate() {
        for (index, _) in row.iter().enumerate() {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
                Transform {
                    translation: Vec3::new(
                        (index as f32 * (TILE_SIZE + TILE_MARGIN)) - TRANSLATION_OFFSET_X,
                        -(row_index as f32 * (TILE_SIZE + TILE_MARGIN)) - TRANSLATION_OFFSET_Y,
                        0.0,
                    ),
                    ..default()
                },
                Text2d("".into()),
                Tile {
                    letter: Option::None,
                    color: Color::WHITE,
                },
                Location {
                    x: index,
                    y: row_index,
                },
            ));
        }
    }
}

fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(
        &mut MeshMaterial2d<ColorMaterial>,
        &Location,
        &mut Text2d,
        &mut Tile,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for key in keyboard_input.get_just_pressed() {
        if let Some(valid_char) = validate_key(*key) {
            if game_state.current_index > 4 {
                println!("out of tiles!");
                return;
            }
            for (material_handle, location, mut text_handle, mut tile) in query.iter_mut() {
                if location.x == game_state.current_index
                    && location.y == game_state.current_row
                    && let Some(material) = materials.get_mut(&material_handle.0)
                {
                    material.color = Color::BLACK;
                    tile.letter = Some(valid_char);
                    text_handle.0 = valid_char.to_string();
                }
            }
            game_state.current_index += 1;
            break;
        }
    }
}

fn validate_key(key: KeyCode) -> Option<char> {
    match key {
        KeyCode::KeyA => Some('a'),
        KeyCode::KeyB => Some('b'),
        KeyCode::KeyC => Some('c'),
        KeyCode::KeyD => Some('d'),
        KeyCode::KeyE => Some('e'),
        KeyCode::KeyF => Some('f'),
        KeyCode::KeyG => Some('g'),
        KeyCode::KeyH => Some('h'),
        KeyCode::KeyI => Some('i'),
        KeyCode::KeyJ => Some('j'),
        KeyCode::KeyK => Some('k'),
        KeyCode::KeyL => Some('l'),
        KeyCode::KeyM => Some('m'),
        KeyCode::KeyN => Some('n'),
        KeyCode::KeyO => Some('o'),
        KeyCode::KeyP => Some('p'),
        KeyCode::KeyQ => Some('q'),
        KeyCode::KeyR => Some('r'),
        KeyCode::KeyS => Some('s'),
        KeyCode::KeyT => Some('t'),
        KeyCode::KeyU => Some('u'),
        KeyCode::KeyV => Some('v'),
        KeyCode::KeyW => Some('w'),
        KeyCode::KeyX => Some('x'),
        KeyCode::KeyY => Some('y'),
        KeyCode::KeyZ => Some('z'),
        _ => {
            println!("not a match");
            None
        }
    }
}
