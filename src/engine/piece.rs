#[derive(Clone, Eq, Hash, Copy, Debug, PartialEq)]
pub(crate) enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl Piece {
    pub(crate) fn parse(piece: &str) -> Self {
        match piece {
            "K" => Piece::King,
            "Q" => Piece::Queen,
            "B" => Piece::Bishop,
            "N" => Piece::Knight,
            "R" => Piece::Rook,
            "P" => Piece::Pawn,
            p => panic!("Invalid piece {}", p),
        }
    }

    pub(crate) fn notation(&self) -> &str {
        match self {
            Piece::King => "K",
            Piece::Queen => "Q",
            Piece::Bishop => "B",
            Piece::Knight => "N",
            Piece::Rook => "R",
            Piece::Pawn => "P",
        }
    }

    pub(crate) fn pretty(&self) -> &str {
        match self {
            Piece::King => "♔",
            Piece::Queen => "♕",
            Piece::Bishop => "♗",
            Piece::Knight => "♘",
            Piece::Rook => "♖",
            Piece::Pawn => "♙",
        }
    }
}

macro_rules! p {
    ($piece:literal) => {
        Piece::parse($piece)
    };
}

pub(crate) use p;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_parse() {
        assert_eq!(p!("K"), Piece::King);
        assert_eq!(p!("Q"), Piece::Queen);
        assert_eq!(p!("B"), Piece::Bishop);
        assert_eq!(p!("N"), Piece::Knight);
        assert_eq!(p!("R"), Piece::Rook);
        assert_eq!(p!("P"), Piece::Pawn);
    }
}
