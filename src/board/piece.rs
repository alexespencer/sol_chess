use strum::{Display, EnumString};

#[derive(Clone, Eq, Hash, Copy, Debug, PartialEq, EnumString, Display)]
pub enum Piece {
    #[strum(serialize = "King", to_string = "K")]
    King,
    #[strum(serialize = "Queen", to_string = "Q")]
    Queen,
    #[strum(serialize = "Bishop", to_string = "B")]
    Bishop,
    #[strum(serialize = "Knight", to_string = "N")]
    Knight,
    #[strum(serialize = "Rook", to_string = "R")]
    Rook,
    #[strum(serialize = "Pawn", to_string = "P")]
    Pawn,
}

impl Piece {
    pub fn pretty(&self) -> String {
        match self {
            Piece::King => "♔",
            Piece::Queen => "♕",
            Piece::Bishop => "♗",
            Piece::Knight => "♘",
            Piece::Rook => "♖",
            Piece::Pawn => "♙",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_parse() {
        assert_eq!(Piece::try_from("K").unwrap(), Piece::King);
        assert_eq!(Piece::try_from("King").unwrap(), Piece::King);
        assert_eq!(Piece::try_from("Q").unwrap(), Piece::Queen);
        assert_eq!(Piece::try_from("Queen").unwrap(), Piece::Queen);
        assert_eq!(Piece::try_from("B").unwrap(), Piece::Bishop);
        assert_eq!(Piece::try_from("Bishop").unwrap(), Piece::Bishop);
        assert_eq!(Piece::try_from("N").unwrap(), Piece::Knight);
        assert_eq!(Piece::try_from("Knight").unwrap(), Piece::Knight);
        assert_eq!(Piece::try_from("R").unwrap(), Piece::Rook);
        assert_eq!(Piece::try_from("Rook").unwrap(), Piece::Rook);
        assert_eq!(Piece::try_from("P").unwrap(), Piece::Pawn);
        assert_eq!(Piece::try_from("Pawn").unwrap(), Piece::Pawn);
    }

    #[test]
    fn test_piece_to_string() {
        assert_eq!(Piece::King.to_string(), "K".to_string());
        assert_eq!(Piece::Queen.to_string(), "Q".to_string());
        assert_eq!(Piece::Bishop.to_string(), "B".to_string());
        assert_eq!(Piece::Knight.to_string(), "N".to_string());
        assert_eq!(Piece::Rook.to_string(), "R".to_string());
        assert_eq!(Piece::Pawn.to_string(), "P".to_string());
    }
}
