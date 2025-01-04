use super::{board::Board, coord::Coord, piece::Piece, square::Square};

#[derive(PartialEq, Hash, Eq, Clone)]
pub(crate) struct Move {
    pub(crate) from: Square,
    pub(crate) to: Square,
}

impl Move {
    pub(crate) fn new(from: Square, to: Square) -> Self {
        Move { from, to }
    }

    pub(crate) fn notation(&self) -> String {
        format!("{} -> {}", self.from.notation(), self.to.notation())
    }
}

macro_rules! mv {
    ($piece:literal, $from:literal, $to:literal, $target:literal) => {
        Move::new(sq!($piece, $from), sq!($target, $to))
    };
}

pub(crate) use mv;
