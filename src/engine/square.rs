use super::{coord::Coord, piece::Piece};

#[derive(Clone, Debug)]
pub(crate) enum Square {
    Empty(Coord),
    Occupied(Piece, Coord),
}

impl Square {
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Square::Empty(_) => true,
            _ => false,
        }
    }

    pub(crate) fn coord(&self) -> Coord {
        match self {
            Square::Empty(coord) => coord.clone(),
            Square::Occupied(_, coord) => coord.clone(),
        }
    }

    pub(crate) fn occupied(&self) -> Option<Piece> {
        match self {
            Square::Occupied(piece, _) => Some(*piece),
            _ => None,
        }
    }
}

macro_rules! sq {
    ($coord:literal) => {
        Square::Empty(at!($coord))
    };
    ($piece:literal, $coord:literal) => {
        Square::Occupied(p!($piece), at!($coord))
    };
}

pub(crate) use sq;
