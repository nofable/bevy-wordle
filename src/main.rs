use bevy::prelude::*;

const ROW_COUNT: usize = 6;
const COL_COUNT: usize = 5;
const TILE_SIZE: f32 = 100.0;
const TILE_MARGIN: f32 = 5.0;
const TRANSLATION_OFFSET_X: f32 = (TILE_SIZE * (COL_COUNT as f32 / 2.0)) + (TILE_MARGIN * 2.0);
const TRANSLATION_OFFSET_Y: f32 = -((TILE_SIZE * (ROW_COUNT as f32 / 2.0)) + (TILE_MARGIN * 2.5));

#[derive(Resource, Clone)]
struct GameState {
    answer: String,
    current_row: usize,
    current_index: usize,
    grid: [[Tile; 5]; 6],
    success: bool,
}

#[derive(Copy, Clone)]
enum TileState {
    Unknown,
    Correct,
    Misplaced,
    Incorrect,
}

#[derive(Clone, Copy, Component)]
struct Tile {
    state: TileState,
    letter: Option<char>,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            state: TileState::Unknown,
            letter: None,
        }
    }
}

impl GameState {
    fn new() -> GameState {
        GameState {
            answer: String::from("hello"),
            current_row: 0,
            current_index: 0,
            grid: [[Tile::new(); COL_COUNT]; ROW_COUNT],
            success: false,
        }
    }

    fn can_add_letter(&self) -> bool {
        self.current_row < ROW_COUNT && self.current_index < COL_COUNT
    }

    fn add_letter(&mut self, c: char) {
        if self.can_add_letter() {
            self.grid[self.current_row][self.current_index].letter = Some(c);
            self.current_index += 1;
        }
    }

    fn can_make_delete(&self) -> bool {
        self.current_index > 0
    }

    fn make_delete(&mut self) {
        if self.can_make_delete() {
            self.grid[self.current_row][self.current_index - 1].letter = None;
            self.current_index -= 1;
        }
    }

    fn check_answer(&mut self) {
        let split: Vec<char> = self.answer.chars().collect();
        let mut guess = self.grid[self.current_row];
        // TODO: this isn't currently writing back to the game state for some reason
        guess
            .iter_mut()
            .zip(split.iter())
            .for_each(|(tile, answer_char)| {
                let guess_char = tile.letter.unwrap();
                if guess_char == *answer_char {
                    tile.state = TileState::Correct;
                } else if split.contains(answer_char) {
                    tile.state = TileState::Misplaced;
                } else {
                    tile.state = TileState::Incorrect;
                }
            });

        let guess_as_string: String = self.grid[self.current_row]
            .iter()
            .filter_map(|&tile| tile.letter)
            .collect();

        self.success = guess_as_string == self.answer;
    }

    fn can_make_guess(&self) -> bool {
        self.current_row < ROW_COUNT && self.current_index >= COL_COUNT
    }

    fn make_guess(&mut self) {
        if self.can_make_guess() {
            self.check_answer();
            if self.success {
                println!("Well done, you got it!");
            } else {
                println!("Not quite!");
                self.current_row += 1;
                self.current_index = 0;
            }
        }
    }
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
    let game_state = GameState::new();
    // build game grid
    for (row_index, row) in game_state.grid.iter().enumerate() {
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
                Tile::new(),
                Location {
                    x: index,
                    y: row_index,
                },
            ));
        }
    }

    commands.insert_resource(game_state);
}

fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(&mut MeshMaterial2d<ColorMaterial>, &Location, &mut Text2d), With<Tile>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let current_row = game_state.current_row;
    let current_index = game_state.current_index;

    if let Some(key) = keyboard_input.get_just_pressed().next() {
        if *key == KeyCode::Enter && game_state.can_make_guess() {
            game_state.make_guess();
            for (material_handle, location, _) in query.iter_mut() {
                if location.y == current_row
                    && let Some(material) = materials.get_mut(&material_handle.0)
                {
                    let x = location.x;

                    let tile_state = game_state.grid[current_row][x].state;
                    match tile_state {
                        TileState::Correct => material.color = Color::Srgba(Srgba::GREEN),
                        TileState::Misplaced => material.color = Color::Srgba(Srgba::RED),
                        TileState::Incorrect => material.color = Color::BLACK,
                        TileState::Unknown => material.color = Color::BLACK,
                    }
                }
            }
        }

        if *key == KeyCode::Backspace && game_state.can_make_delete() {
            game_state.make_delete();
            for (material_handle, location, mut text_handle) in query.iter_mut() {
                if location.x == current_index - 1
                    && location.y == current_row
                    && let Some(material) = materials.get_mut(&material_handle.0)
                {
                    material.color = Color::WHITE;
                    text_handle.0 = "".into();
                }
            }
        }

        if let Some(valid_char) = validate_key(*key)
            && game_state.can_add_letter()
        {
            game_state.add_letter(valid_char);
            for (material_handle, location, mut text_handle) in query.iter_mut() {
                if location.x == current_index
                    && location.y == current_row
                    && let Some(material) = materials.get_mut(&material_handle.0)
                {
                    material.color = Color::BLACK;
                    text_handle.0 = valid_char.to_string();
                }
            }
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
