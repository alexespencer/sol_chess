use super::{coord::Coord, piece::Piece};

pub(crate) struct Move {
    piece: Piece,
    from: Coord,
    to: Coord,
    target: Piece,
}

impl Move {
    pub(crate) fn new(piece: Piece, from: Coord, to: Coord, target: Piece) -> Self {
        Move {
            piece,
            from,
            to,
            target,
        }
    }

    pub(crate) fn notation(&self) -> String {
        format!(
            "{}{}{}",
            self.piece.notation(),
            self.from.notation,
            self.to.notation
        )
    }
}
