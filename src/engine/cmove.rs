use super::{board::Board, piece::Piece, square::Square};

#[derive(PartialEq, Hash, Eq, Clone)]
pub(crate) struct CMove {
    pub(crate) from_piece: Piece,
    pub(crate) from: Square,
    pub(crate) to_piece: Piece,
    pub(crate) to: Square,

    // Used to disambiguate when looking at notation
    disambig: String,
}

impl CMove {
    pub(crate) fn new(from: Square, to: Square) -> Self {
        let qualifier = String::from("");
        let from_piece = from.piece.expect("Trying to move a blank");
        let to_piece = to.piece.expect("Trying to capture a blank");
        CMove {
            from_piece,
            from,
            to_piece,
            to,
            disambig: "".to_string(),
        }
    }

    pub(crate) fn notation(&self) -> String {
        let piece_qualifier = match &self.from_piece {
            Piece::Pawn => self.from.file_notation(),
            p => p.notation(),
        };
        format!(
            "{}{}x{}",
            piece_qualifier,
            self.disambig,
            self.to.notation()
        )
    }
}

macro_rules! mv {
    ($from:literal, $to:literal) => {{
        CMove::new(sq!($from), sq!($to))
    }};
}

pub(crate) use mv;
