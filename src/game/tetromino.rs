use ratatui::style::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoType {
    pub const ALL: [TetrominoType; 7] = [
        Self::I,
        Self::O,
        Self::T,
        Self::S,
        Self::Z,
        Self::J,
        Self::L,
    ];

    pub fn color(self) -> Color {
        match self {
            Self::I => Color::Cyan,
            Self::O => Color::Yellow,
            Self::T => Color::Magenta,
            Self::S => Color::Green, 
            Self::Z => Color::Red,
            Self::J => Color::Blue,
            Self::L => Color::Rgb(255,165,0), //orange
        }
    }
    /// Bounding box drawn as text: '#' = filled.
    /// Row count == box size.
    fn shape(self) -> &'static [&'static str] {
        match self {
            Self::I => &["....","####", "....", "...."],
            Self::O => &["##","##"],
            Self::T => &[".#.", "###","..."],
            Self::S => &[".##","##.","..."],
            Self::Z => &["##.",".##","..."],
            Self::J => &["#..", "###","..."],
            Self::L => &["..#", "###", "..."],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub kind: TetrominoType,
    pub size: usize, 
    cells: [[bool; 4];4],
}

impl Piece {
    pub fn new(kind: TetrominoType) -> Self {
        let rows = kind.shape();
        let mut cells = [[false; 4];4];
        for (r, row) in rows.iter().enumerate() {
            for (c, ch) in row.chars().enumerate() {
                cells[r][c] = ch == '#';
            }
        }
        Self {
            kind,
            size: rows.len(),
            cells,
        }
    }
    
    /// 90° clockwise rotation inside the piece's own NxN bounding box.
    pub fn rotated_cw(&self) -> Self {
        let n = self.size;
        let mut cells = [[false; 4]; 4];
        for (r, row) in cells.iter_mut().enumerate().take(n) {
            for (c, cell) in row.iter_mut().enumerate().take(n) {
                *cell = self.cells[n - 1 - c][r];
            }
        }
        Self {
            kind: self.kind,
            size: n,
            cells,
        }
    }
    
    /// Offsets `(dx, dy)` of the 4filled cells relative to the piece origin.
    pub fn blocks(self) -> impl Iterator<Item = (i32, i32)> {
        (0..4).flat_map(move |r| {
            (0..4).filter_map(move |c| self.cells[r][c].then_some((c as i32, r as i32)))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_piece_has_four_blocks() {
        for kind in TetrominoType::ALL {
            assert_eq!(Piece::new(kind).blocks().count(), 4, "{kind:?}");
        }
    }

    #[test]
    fn bounding_box_sizes() {
        assert_eq!(Piece::new(TetrominoType::I).size, 4);
        assert_eq!(Piece::new(TetrominoType::O).size, 2);
        assert_eq!(Piece::new(TetrominoType::T).size, 3);
    }

    #[test]
    fn four_rotations_return_to_start() {
        for kind in TetrominoType::ALL {
            let p = Piece::new(kind);
            let r4 = p.rotated_cw()
                .rotated_cw()
                .rotated_cw()
                .rotated_cw();
            assert_eq!(p, r4, "{kind:?}" );
        }
    }

    #[test]
    fn o_piece_is_rotation_invariant() {
        let o = Piece::new(TetrominoType::O);
        assert_eq!(o, o.rotated_cw() );
    }

    #[test]
    fn t_piece_points_right_after_one_rotation() {
        let t = Piece::new(TetrominoType::T).rotated_cw();
        let mut blocks: Vec<_> = t.blocks().collect();
        blocks.sort();
        assert_eq!(blocks, vec![(1, 0), (1, 1), (1, 2), (2, 1)]);
    }
}
