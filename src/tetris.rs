use std::fmt::Display;
use rand::{prelude::Distribution, distributions::Standard, Rng};

// const PLAY_FIELD_ANCHOR: usize = 2;
const PLAY_FIELD_HEIGHT: usize = 24;
const PLAY_FIELD_WIDTH: usize = 10;
const PLAY_FIELD_BUFFER: usize = 4;
const PLAY_FIELD_BUFFER_ICON: &'static str = "⬜";
const PLAY_FIELD_ICON: &'static str = "⬛";
const NEXT_FIELD_HEIGHT: usize = 5;
const NEXT_FIELD_WIDTH: usize = 5;


#[derive(Debug, Clone)]
pub enum TetrominoNames { T, L, J, O, I, S, Z }

impl Distribution<TetrominoNames> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> TetrominoNames {
        match rng.gen_range(0..=6) {
            0 => TetrominoNames::T,
            1 => TetrominoNames::L,
            2 => TetrominoNames::J,
            3 => TetrominoNames::O,
            4 => TetrominoNames::I,
            5 => TetrominoNames::S,
            _ => TetrominoNames::Z
        }
    }
}

impl TetrominoNames {
    pub fn get_position(&self) -> [(usize, usize); 4] {
        match &self {
            TetrominoNames::T => [(2, 5), (2, 4), (2, 3), (3, 4)],
            TetrominoNames::L => [(2, 5), (2, 4), (2, 3), (3, 3)],
            TetrominoNames::J => [(2, 5), (2, 4), (2, 3), (1, 3)],
            TetrominoNames::O => [(2, 5), (2, 4), (3, 5), (3, 4)],
            TetrominoNames::I => [(2, 5), (2, 4), (2, 3), (2, 2)],
            TetrominoNames::S => [(2, 5), (2, 4), (3, 4), (3, 3)],
            TetrominoNames::Z => [(2, 5), (2, 4), (1, 4), (1, 3)],
        }
    }

    pub fn get_icon(&self) -> &'static str {
        match &self {
            TetrominoNames::T => "🟥",
            TetrominoNames::L => "🟧",
            TetrominoNames::J => "🟨",
            TetrominoNames::O => "🟩",
            TetrominoNames::I => "🟦",
            TetrominoNames::S => "🟪",
            TetrominoNames::Z => "🟫",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub name: TetrominoNames,
    position: [(usize, usize); 4],
    icon: &'static str
}

impl Tetromino {
    /// Create a new tetromino
    pub fn new(name: TetrominoNames) -> Tetromino {
        let position = name.get_position();
        let icon = name.get_icon();
        Tetromino { name, position, icon }
    }

    /// Create a new random tetromino
    pub fn random() -> Tetromino {
        let name: TetrominoNames = rand::random();
        let position = name.get_position();
        let icon = name.get_icon();
        let mut tetromino = Tetromino { name, position, icon };
        let mut rng = rand::thread_rng();
        let n_rotation = rng.gen_range(0..=3);
        for _ in 0..=n_rotation { tetromino.rotate(true); };
        tetromino
    }

    /// Rotating tetromino -90 degree
    pub fn rotate(&mut self, in_place: bool) -> [(usize, usize); 4] {
        let (ox, oy) = self.position[1];
        let angle = -90.0_f32.to_radians();
        let sin = angle.sin() as i8;
        let cos = angle.cos() as i8;
        let mut new_shape = vec![];
        for (px, py) in self.position {
            let qx: i8 = ox as i8 + (cos * (px as i8 - ox as i8)) - (sin * (py as i8 - oy as i8));
            let qy: i8 = oy as i8 + (sin * (px as i8 - ox as i8)) + (cos * (py as i8 - oy as i8));
            new_shape.push((qx as usize, qy as usize));
        }
        if in_place {
            self.position = new_shape.clone().try_into().unwrap();
        }
        new_shape.try_into().unwrap()
    }

    pub fn include(&self, position: (usize, usize)) -> bool {
        self.position.iter().any(|pos| { pos == &position })
    }
}

pub(crate) trait TetrisField {
    /// Create a new Field
    fn new() -> Self;

    /// Set cell in the Field
    fn set_cell(&mut self, row: usize, col: usize, value: &'static str);

    /// Add tetromino to the playfield
    fn spawn(&mut self, tetromino: Tetromino) {
        for (row, col) in tetromino.position {
            self.set_cell(row, col, tetromino.icon)
        }
    }

    /// Remove tetromino from the playfield
    fn remove(&mut self, cells: Vec<(usize, usize)>) {
        for (row, col) in cells {
            if row < PLAY_FIELD_BUFFER {
                self.set_cell(row, col, PLAY_FIELD_BUFFER_ICON)
            } else {
                self.set_cell(row, col, PLAY_FIELD_ICON)
            }
        }
    }
}

pub struct PlayField {
    cells: Vec<Vec<&'static str>>
}

impl Display for PlayField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in &self.cells {
            for x in y {
                write!(f, "{}", x)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl TetrisField for PlayField {
    /// Creating a new empty playfield
    fn new() -> PlayField {
        let cells =
            (0..PLAY_FIELD_HEIGHT).map(|y| {
                (0..PLAY_FIELD_WIDTH).map(|_x| {
                    if y < PLAY_FIELD_BUFFER { PLAY_FIELD_BUFFER_ICON } else { PLAY_FIELD_ICON }
                }).collect()
            }).collect();
        PlayField { cells }
    }

    /// Set cell in the field
    fn set_cell(&mut self, row: usize, col: usize, value: &'static str) {
        self.cells[row][col] = value;
    }
}

impl PlayField {
    /// Get filled row in playfield
    pub fn get_filled_row(&self) -> Vec<usize> {
        let mut filled_row = vec![];
        for (x, row) in self.cells.iter().enumerate() {
            if x < 4 { continue; }
            if !row.iter().any(|&icon| { icon == PLAY_FIELD_ICON }) {
                filled_row.push(x);
            }
        }
        filled_row.sort();
        filled_row
    }

    /// Crush filled row and move above rows down
    pub fn crush_row(&mut self, row_idx: usize) {
        let mut subtractor = 0_usize;
        loop {
            let idx = row_idx - subtractor;
            // Stop the loop when it reach the buffer row
            if (idx) < 4 { break; }
            // Change the toppest row to playfield icon (empty)
            if (idx) == 4 {
                self.cells[idx] = vec![PLAY_FIELD_ICON; PLAY_FIELD_WIDTH];
            } else {
                self.cells[idx] = self.cells[idx - 1].clone();
            }
            subtractor += 1;
        }
    }
}

pub struct Nextfield {
    cells: Vec<Vec<&'static str>>
}

impl Display for Nextfield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in &self.cells {
            for x in y {
                write!(f, "{}", x)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl TetrisField for Nextfield {
    /// Create new Nextfield
    fn new() -> Nextfield {
        let cells =
            (0..NEXT_FIELD_HEIGHT).map(|_y| {
                (0..NEXT_FIELD_WIDTH).map(|_x| {
                    PLAY_FIELD_ICON
                }).collect()
            }).collect();
        Nextfield { cells }
    }

    /// Set cell in the field
    fn set_cell(&mut self, row: usize, col: usize, value: &'static str) {
        self.cells[row][col] = value;
    }
}

pub struct Game {
    pub over: bool,
    score: u32,
    playfield: PlayField,
    current_tetromino: Tetromino,
    nextfield: Nextfield,
    next_tetromino: Tetromino
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (x, row) in self.playfield.cells.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if self.current_tetromino.include((x, y)) {
                    write!(f, "{}", self.current_tetromino.icon)?;
                } else {
                    write!(f, "{}", col)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Game {
    /// Create a new game
    pub fn new() -> Game {
        let over = false;
        let score = 0;
        let playfield = PlayField::new();
        let current_tetromino = Tetromino::random();
        let nextfield = Nextfield::new();
        let next_tetromino = Tetromino::random();
        Game { over, score, playfield, current_tetromino, nextfield, next_tetromino }
    }

    /// Check if the current tetromino can be moved down
    pub fn can_move_down(&self) -> bool {
        let bottom = self.current_tetromino.position
            .iter()
            .map(|(x, _)| { x })
            .max()
            .unwrap();

        // Check if tetromino already on the bottom
        if bottom == &(PLAY_FIELD_HEIGHT - 1) { return  false }

        let mut bottom_cells: Vec<(usize, usize)> = vec![];
        for (x, y) in self.current_tetromino.position {
            if bottom_cells.iter().any(|(_a, b)| { b == &y }) {
                if let Some(idx) = bottom_cells.iter().position(|(a, b)| { b == &y && a < &x }) {
                    bottom_cells.remove(idx);
                    bottom_cells.push((x, y));
                }
            } else {
                bottom_cells.push((x, y));
            }
        }

        // Check if there are any existing tetromino below the current tetromino
        for (x, y) in bottom_cells {
            if self.playfield.cells[x + 1][y] != PLAY_FIELD_BUFFER_ICON
            && self.playfield.cells[x + 1][y] != PLAY_FIELD_ICON {
                return false;
            }
        }
        true
    }

    /// Check if the current tetromino can be moved right
    pub fn can_move_right(&self) -> bool {
        let right = self.current_tetromino.position
            .iter()
            .map(|(_, y)| { y })
            .max()
            .unwrap();

        // Check if tetromino already on the right side edge
        if right == &(PLAY_FIELD_WIDTH - 1) { return false }

        let mut right_cells: Vec<(usize, usize)> = vec![];
        for (x, y) in self.current_tetromino.position {
            if right_cells.iter().any(|(a, _b)| { a == &x }) {
                if let Some(idx) = right_cells.iter().position(|(a, b)| { a == &x && b < &y }) {
                    right_cells.remove(idx);
                    right_cells.push((x, y));
                }
            } else {
                right_cells.push((x, y));
            }
        }

        // Check if there are any existing tetromino on the right side of the current tetromino
        for (x, y) in right_cells {
            if self.playfield.cells[x][y + 1] != PLAY_FIELD_BUFFER_ICON
            && self.playfield.cells[x][y + 1] != PLAY_FIELD_ICON {
                return false;
            }
        }
        true
    }

    /// Check if the current tetromino can be moved left
    pub fn can_move_left(&self) -> bool {
        let left = self.current_tetromino.position
            .iter()
            .map(|(_, y)| { y })
            .min()
            .unwrap();

        // Check if tetromino already on the left side edge
        if left == &0_usize { return false }

        let mut left_cells: Vec<(usize, usize)> = vec![];
        for (x, y) in self.current_tetromino.position {
            if left_cells.iter().any(|(a, _b)| { a == &x }) {
                if let Some(idx) = left_cells.iter().position(|(a, b)| { a == &x && b > &y }) {
                    left_cells.remove(idx);
                    left_cells.push((x, y));
                }
            } else {
                left_cells.push((x, y));
            }
        }

        // Check if there are any existing tetromino on the left side of the current tetromino
        for (x, y) in left_cells {
            if self.playfield.cells[x][y - 1] != PLAY_FIELD_BUFFER_ICON
            && self.playfield.cells[x][y - 1] != PLAY_FIELD_ICON {
                return false;
            }
        }
        true
    }

    /// Check if the current tetromino can be rotated
    pub fn can_rotate(&mut self) -> bool {
        let rotated_position = self.current_tetromino.rotate(false);
        for (x, y) in rotated_position {
            // Check if after rotating, the tetromino is out of playfield
            if x > 23 || y > 9 { return false }
            // Check if after rotating, the tetromino is on another existing tetromino
            if self.playfield.cells[x][y] != PLAY_FIELD_BUFFER_ICON
            && self.playfield.cells[x][y] != PLAY_FIELD_ICON {
                return false;
            }
        }
        true
    }

    /// Move the current tetromino down a cell
    pub fn move_down(&mut self) {
        self.current_tetromino.position = self.current_tetromino.position
            .map(|(row, col)| { (row + 1, col)});
    }

    /// Move the current tetromino right a cell
    pub fn move_right(&mut self) {
        self.current_tetromino.position = self.current_tetromino.position
            .map(|(row, col)| { (row, col + 1)});
    }

    /// Move the current tetromino left a cell
    pub fn move_left(&mut self) {
        self.current_tetromino.position = self.current_tetromino.position
            .map(|(row, col)| { (row, col - 1)});
    }

    /// Rotate the current tetromino -90 degree
    pub fn rotate(&mut self) {
        self.current_tetromino.rotate(true);
    }

    /// Check if the tetromino stack has touch the top
    pub fn touch_the_top(&self) -> bool {
        self.playfield.cells[4].iter().any(|&cell| {
            cell != PLAY_FIELD_BUFFER_ICON && cell != PLAY_FIELD_ICON
        })
    }

    /// The Game tick
    pub fn tick(&mut self) {
        // If the game is over do nothing
        if self.over { return; }

        // Check if the tetromino has touch the top
        if self.touch_the_top() {
            self.over = true;
            return;
        }

        // Check for moving done
        if self.can_move_down() {
            self.move_down();
        } else {
            self.playfield.spawn(self.current_tetromino.clone());
            for (nth, &row) in self.playfield.get_filled_row().iter().enumerate() {
                self.crush_row(row, nth);
            }
            self.current_tetromino = self.next_tetromino.clone();
            self.next_tetromino = Tetromino::random();
        }
    }

    /// Crush the filled row and update the score
    pub fn crush_row(&mut self, row_idx: usize, nth: usize) {
        let bonus_score = if nth == 1 || nth == 2 { 10 } else if nth == 3 { 20 } else { 0 };
        self.playfield.crush_row(row_idx);
        self.score += 10 + bonus_score;
    }

    /// Get the current score
    pub fn get_score(&self) -> u32 {
        self.score
    }

    /// Print the playfield with the current tetromino
    pub fn print(&self) {
        for (x, row) in self.playfield.cells.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if self.current_tetromino.include((x, y)) {
                    print!("{}", self.current_tetromino.icon);
                } else {
                    print!("{}", col);
                }
            }
            print!("\n");
        }
    }

    /// Print the nextfield with the next tetromino
    pub fn get_nextfield_str(&self) -> String {
        let mut nextfield_str = String::new();
        for (x, row) in self.nextfield.cells.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if self.next_tetromino.include((x, y + 2)) {
                    nextfield_str.push_str(self.next_tetromino.icon);
                } else {
                    nextfield_str.push_str(col);
                }
            }
            nextfield_str.push_str("\n");
        }
        nextfield_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crushing_filled_row() {
        let mut game = Game::new();
        game.playfield.cells[20] = vec![TetrominoNames::T.get_icon(); PLAY_FIELD_WIDTH];
        game.playfield.cells[21] = vec![TetrominoNames::T.get_icon(); PLAY_FIELD_WIDTH];
        game.playfield.cells[22] = vec![TetrominoNames::L.get_icon(); PLAY_FIELD_WIDTH];
        game.playfield.cells[23] = vec![TetrominoNames::O.get_icon(); PLAY_FIELD_WIDTH];
        for (nth, &row) in game.playfield.get_filled_row().iter().enumerate() {
            game.crush_row(row, nth);
        }
        assert_eq!(game.playfield.cells[20], vec![PLAY_FIELD_ICON; PLAY_FIELD_WIDTH]);
        assert_eq!(game.playfield.cells[21], vec![PLAY_FIELD_ICON; PLAY_FIELD_WIDTH]);
        assert_eq!(game.playfield.cells[22], vec![PLAY_FIELD_ICON; PLAY_FIELD_WIDTH]);
        assert_eq!(game.playfield.cells[23], vec![PLAY_FIELD_ICON; PLAY_FIELD_WIDTH]);
        assert_eq!(game.get_score(), 80);
    }

}