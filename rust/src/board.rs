use crate::piece::Colour;

use crate::piece::Piece;

pub struct Board {
    pub squares: Vec<Square>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: vec![Square {
                pos_x: 1,
                pos_y: 1,
                piece: Some(Piece::new_rook(Colour::White)),
            }],
        }
    }
}

pub struct Square {
    pub pos_x: u8,
    pub pos_y: u8,
    pub piece: Option<Piece>,
}
