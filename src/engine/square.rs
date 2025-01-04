use super::{coord::Coord, piece::Piece};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
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

    pub(crate) fn coord_ref(&self) -> &Coord {
        match self {
            Square::Empty(coord) => coord,
            Square::Occupied(_, coord) => coord,
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

    pub(crate) fn notation(&self) -> String {
        match self {
            Square::Empty(coord) => coord.notation.clone(),
            Square::Occupied(piece, coord) => {
                format!("{}{}", piece.notation(), coord.notation.clone())
            }
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
