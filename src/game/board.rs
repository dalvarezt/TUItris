use ratatui::style::Color;

use super::tetromino::Piece;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 22;
pub const BUFFER_ROWS: usize = 2;

type Row = [Option<Color>; WIDTH];

#[derive(Clone, Debug)]
pub struct Board {
    cells: [Row; HEIGHT],
}

impl Board {
    pub fn new () -> Self {
        Self {
            cells: [[None; WIDTH]; HEIGHT],
        }
    }

    /// Color of the locked block at (x, y), if any. Panics if out of range
    pub fn cell(&self, x: usize, y: usize) -> Option<Color> {
        self.cells[y][x]
    }

    fn index(x: i32, y: i32) -> Option<(usize, usize)> {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        (x < WIDTH && y < HEIGHT).then_some((x, y))
    }

    /// In bounds and not occupied
    fn is_free(&self, x: i32, y: i32) -> bool {
        matches!(Self::index(x, y), Some((ix, iy)) if self.cells[iy][ix].is_none())
    }

    ///True if every block of `piece` fits when its origin is placed at (x, y)
    pub fn is_valid_position(&self, piece: &Piece, x: i32, y: i32) -> bool {
        piece.blocks().all(|(dx, dy)| self.is_free(x + dx, y + dy))
    }

    /// Writes the piece's blocks into the grid using its color
    pub fn lock_piece(&mut self, piece: &Piece, x: i32, y: i32) {
        let color = piece.kind.color();
        for (dx, dy) in piece.blocks() {
            if let Some((ix, iy)) = Self::index(x + dx, y + dy) {
                self.cells[iy][ix] = Some(color);
            }
        }
    }

    pub fn clear_lines(&mut self) -> usize {
        let remaining: Vec<Row> = self
            .cells
            .iter()
            .filter(|row| row.iter().any(Option::is_none))
            .copied()
            .collect();
        let cleared = HEIGHT - remaining.len();

        let mut cells: [Row; HEIGHT] = [[None; WIDTH]; HEIGHT];
        cells[cleared..].copy_from_slice(&remaining);
        self.cells = cells;
        cleared
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::tetromino::TetrominoType;

    fn fill_row(board: &mut Board, y: usize) {
        for x in 0..WIDTH {
            board.cells[y][x] = Some(Color::Red);
        }
    }

    #[test]
    fn valid_inside_board_invalid_outside() {
        let board = Board::new();
        let t = Piece::new(TetrominoType::T);
        assert!(board.is_valid_position(&t, 3, 0));
        assert!(!board.is_valid_position(&t, -1, 0)); //left wall
        assert!(board.is_valid_position(&t, 7, 0));
        assert!(!board.is_valid_position(&t, 8, 0)); //right wall
        assert!(board.is_valid_position(&t, 3, 20));
        assert!(!board.is_valid_position(&t, 3, 21)); //floor
    }

    #[test]
    fn locked_blocks_collide() {
        let mut board = Board::new();
        let o = Piece::new(TetrominoType::O);
        board.lock_piece(&o, 4, 20);
        assert_eq!(board.cell(4, 20), Some(TetrominoType::O.color()));
        assert!(!board.is_valid_position(&o, 4, 20));
        assert!(!board.is_valid_position(&o, 5, 19));
        assert!(board.is_valid_position(&o, 4, 18));
    }

    #[test]
    fn clear_single_line_shifts_rows_down() {
        let mut board = Board::new();
        fill_row(&mut board, 21);
        board.cells[20][0] = Some(Color::Blue);

        assert_eq!(board.clear_lines(), 1);
        assert_eq!(board.cell(0, 21), Some(Color::Blue));
        assert_eq!(board.cell(1, 21), None);
        assert_eq!(board.cell(0, 20), None);
    }

    #[test]
    fn clear_four_lines() {
        let mut board = Board::new();
        for y in 18..22 {
            fill_row(&mut board, y);
        }

        assert_eq!(board.clear_lines(), 4);
        assert!((0..HEIGHT).all(|y| (0..WIDTH).all(|x| board.cell(x, y).is_none())));
    }

    #[test]
    fn no_full_rows_clears_nothing() {
        let mut board = Board::new();
        board.cells[21][0] = Some(Color::Red);
        assert_eq!(board.clear_lines(), 0);
        assert_eq!(board.cell(0, 21), Some(Color::Red));
    }
}
