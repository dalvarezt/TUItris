use super::board::Board;
use super::tetromino::{Piece, TetrominoType};

/// In place, left 1, right 1, up 1 (dy = -1 is *up*: y grows downward)
const DEFAULT_KIKCS: &[(i32, i32)] = &[
    (0, 0),
    (-1, 0),
    (1, 0), 
    (0, -1)
];

/// The I piece additionally tries +/-2 horizontally befire kicking up 
const I_KICKS: &[(i32, i32)] = &[
    (0, 0),
    (-1, 0),
    (1, 0), 
    (-2, 0), 
    (2, 0), 
    (0, -1)
];

/// Rotates `Piece` clockwise at origin (x, y)
/// Returns the rotated piece and its (possibly kicked) origin,
/// or `None` if every kick collides (rotation cancelled).
pub fn try_rotate_cw(board: &Board, piece: &Piece, x: i32, y: i32) -> Option <(Piece, i32, i32)> {
    let rotated = piece.rotated_cw();
    let kicks = match piece.kind {
        TetrominoType::I => I_KICKS,
        _ => DEFAULT_KIKCS,
    };
    kicks.iter().find_map(|&(dx, dy)| {
        board
            .is_valid_position(&rotated, x + dx, y + dy)
            .then_some((rotated, x + dx, y + dy))
    })
}

 #[cfg(test)]
  mod tests {
      use super::*;

      fn bounds(piece: &Piece, x: i32, _y: i32) -> (i32, i32) {
          let xs: Vec<i32> = piece.blocks().map(|(dx, _)| x + dx).collect();
          (*xs.iter().min().unwrap(), *xs.iter().max().unwrap())
      }

      #[test]
      fn rotates_in_place_when_free() {
          let board = Board::new();
          let t = Piece::new(TetrominoType::T);
          let (rotated, x, y) = try_rotate_cw(&board, &t, 3, 5).unwrap();
          assert_eq!((x, y), (3, 5));
          assert_eq!(rotated, t.rotated_cw());
      }

      #[test]
      fn kicks_off_left_wall() {
          let board = Board::new();
          // vertical I occupying column 0 (its filled column is dx = 2)
          let vertical = Piece::new(TetrominoType::I).rotated_cw();
          let (rotated, x, y) = try_rotate_cw(&board, &vertical, -2, 5).unwrap();
          assert_eq!((x, y), (0, 5)); // needed a +2 kick
          let (min_x, max_x) = bounds(&rotated, x, y);
          assert!(min_x >= 0 && max_x <= 3);
      }

      #[test]
      fn kicks_off_right_wall() {
          let board = Board::new();
          // vertical I occupying column 9
          let vertical = Piece::new(TetrominoType::I).rotated_cw();
          let (rotated, x, y) = try_rotate_cw(&board, &vertical, 7, 5).unwrap();
          assert_eq!((x, y), (6, 5)); // -1 kick
          let (_, max_x) = bounds(&rotated, x, y);
          assert_eq!(max_x, 9);
      }

      #[test]
      fn kicks_up_off_floor() {
          let board = Board::new();
          // flat T on the bottom two rows; its rotated form is 3 tall
          let t = Piece::new(TetrominoType::T);
          let (_, x, y) = try_rotate_cw(&board, &t, 3, 20).unwrap();
          assert_eq!((x, y), (3, 19));
      }

      #[test]
      fn rotation_cancelled_when_all_kicks_fail() {
          let board = Board::new();
          // horizontal I on the floor row: the vertical form needs 2 rows of lift
          let i = Piece::new(TetrominoType::I);
          assert!(try_rotate_cw(&board, &i, 3, 20).is_none());
      }

      #[test]
      fn kicks_away_from_locked_stack() {
          let mut board = Board::new();
          let o = Piece::new(TetrominoType::O);
          board.lock_piece(&o, 4, 12); // blocks at x 4..=5, y 12..=13
          let t = Piece::new(TetrominoType::T);
          // T at (3, 10) is valid; rotated it needs cell (4, 12) -> blocked in place,
          // so the (-1, 0) kick to origin (2, 10) is the first that fits.
          assert!(board.is_valid_position(&t, 3, 10));
          let (rotated, x, y) = try_rotate_cw(&board, &t, 3, 10).unwrap();
          assert_eq!((x, y), (2, 10));
          assert!(board.is_valid_position(&rotated, x, y));
      }

  }

