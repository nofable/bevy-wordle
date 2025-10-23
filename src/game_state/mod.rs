mod answers;
use answers::random_answer;
use bevy::prelude::{Component, Resource};

pub const ROW_COUNT: usize = 6;
pub const COL_COUNT: usize = 5;
pub const TILE_SIZE: f32 = 100.0;
pub const TILE_MARGIN: f32 = 5.0;
pub const TRANSLATION_OFFSET_X: f32 = (TILE_SIZE * (COL_COUNT as f32 / 2.0)) + (TILE_MARGIN * 2.0);
pub const TRANSLATION_OFFSET_Y: f32 =
    -((TILE_SIZE * (ROW_COUNT as f32 / 2.0)) + (TILE_MARGIN * 2.5));

#[derive(Resource, Clone)]
pub struct GameState {
    pub answer: String,
    pub current_row: usize,
    pub current_index: usize,
    pub grid: Vec<Vec<Tile>>,
    pub success: bool,
}

#[derive(Copy, Clone)]
pub enum TileState {
    Unknown,
    Correct,
    Misplaced,
    Incorrect,
}

#[derive(Clone, Copy, Component)]
pub struct Tile {
    pub state: TileState,
    letter: Option<char>,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            state: TileState::Unknown,
            letter: None,
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            answer: random_answer(),
            current_row: 0,
            current_index: 0,
            grid: vec![vec![Tile::new(); COL_COUNT]; ROW_COUNT],
            success: false,
        }
    }

    pub fn can_add_letter(&self) -> bool {
        self.current_row < ROW_COUNT && self.current_index < COL_COUNT
    }

    pub fn add_letter(&mut self, c: char) {
        if self.can_add_letter() {
            self.grid[self.current_row][self.current_index].letter = Some(c);
            self.current_index += 1;
        }
    }

    pub fn can_make_delete(&self) -> bool {
        self.current_index > 0
    }

    pub fn make_delete(&mut self) {
        if self.can_make_delete() {
            self.grid[self.current_row][self.current_index - 1].letter = None;
            self.current_index -= 1;
        }
    }

    fn check_answer(&mut self) {
        let split_answer: Vec<char> = self.answer.chars().collect();
        let mut guess = self.grid[self.current_row].clone();
        let annotated_row = guess
            .iter_mut()
            .zip(split_answer.iter())
            .map(|(tile, answer_char)| {
                let guess_char = tile.letter.unwrap();
                if guess_char == *answer_char {
                    Tile {
                        letter: tile.letter,
                        state: TileState::Correct,
                    }
                } else if split_answer.contains(&guess_char) {
                    Tile {
                        letter: tile.letter,
                        state: TileState::Misplaced,
                    }
                } else {
                    Tile {
                        letter: tile.letter,
                        state: TileState::Incorrect,
                    }
                }
            })
            .collect();
        self.grid[self.current_row] = annotated_row;
        let guess_as_string: String = self.grid[self.current_row]
            .iter()
            .filter_map(|&tile| tile.letter)
            .collect();

        self.success = guess_as_string == self.answer;
    }

    pub fn can_make_guess(&self) -> bool {
        self.current_row < ROW_COUNT && self.current_index >= COL_COUNT
    }

    pub fn make_guess(&mut self) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let gs = GameState::new();
        assert_eq!(gs.current_index, 0);
        assert_eq!(gs.current_row, 0);
    }
}
