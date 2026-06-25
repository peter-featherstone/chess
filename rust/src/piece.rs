use crate::piece::PieceType::Rook;

pub struct Piece {
    piece_type: PieceType,
    colour: Colour,
}

pub enum Colour {
    White,
    Black,
}

enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

impl Piece {
    pub fn new_rook(colour: Colour) -> Piece {
        Self {
            piece_type: Rook,
            colour: colour,
        }
    }
}
