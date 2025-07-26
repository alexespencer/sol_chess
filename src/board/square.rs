use super::constants::BOARD_SIZE;
use super::piece::Piece;
use eyre::{Result, ensure};

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct Location {
    file: u8,
    rank: u8,
}

impl Location {
    pub fn file(&self) -> u8 {
        self.file
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }

    pub fn new(file: u8, rank: u8) -> Self {
        Location { file, rank }
    }

    pub fn file_notation(&self) -> String {
        // TODO: remove unwrap. Check on init that n file <= 3 (or board size)
        String::from("abcd".chars().nth(self.file() as usize).unwrap())
    }

    pub fn rank_notation(&self) -> String {
        format!("{}", BOARD_SIZE - self.rank)
    }

    pub fn notation(&self) -> String {
        format!("{}{}", self.file_notation(), self.rank_notation())
    }
}

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct Square {
    location: Location,
    piece: Option<Piece>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SquarePair {
    start: Square,
    end: Square,
}

impl Square {
    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn piece(&self) -> Option<Piece> {
        self.piece
    }

    pub fn new(location: Location, piece: Option<Piece>) -> Self {
        Square { location, piece }
    }

    pub fn parse(notation: &str) -> Self {
        let mut chars = notation.chars();
        let piece = chars.next().expect("Piece missing");
        let piece = match piece {
            '.' => None,
            c => Piece::try_from(c.to_string().as_str()).ok(),
        };
        let file = chars.next().expect("File missing");
        let file = match file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            _ => panic!("file should be between a-d"),
        };

        let rank = chars.next().unwrap().to_digit(10).expect("rank missing") as u8;
        if rank < 1 || rank > BOARD_SIZE {
            panic!("rank should be between 1-{}", BOARD_SIZE);
        }
        let rank = BOARD_SIZE - rank;
        Square::new(Location::new(file, rank), piece)
    }

    pub fn notation(&self) -> String {
        format!("{}{}", self.piece_notation(), self.location().notation(),)
    }

    pub fn is_occupied(&self) -> bool {
        self.piece().is_some()
    }

    fn piece_notation(&self) -> String {
        match self.piece {
            Some(piece) => piece.to_string(),
            None => "".to_string(),
        }
    }

    pub fn set_piece(&mut self, piece: Option<Piece>) {
        self.piece = piece;
    }
}

impl SquarePair {
    pub fn start(&self) -> &Square {
        &self.start
    }

    pub fn end(&self) -> &Square {
        &self.end
    }

    pub fn dx(&self) -> isize {
        self.end.location().file() as isize - self.start.location().file() as isize
    }

    pub fn dy(&self) -> isize {
        self.end.location().rank() as isize - self.start.location().rank() as isize
    }

    pub fn try_new(start: Square, end: Square) -> Result<Self> {
        ensure!(
            start.location() != end.location(),
            "start position must be different to end position"
        );
        Ok(SquarePair { start, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! validate_square {
        ($notation:literal, $file:expr, $rank:expr) => {
            let notation = format!("{}{}", "K", $notation);
            let square = Square::parse(&notation);
            assert_eq!(square.location().file(), $file);
            assert_eq!(square.location().rank(), $rank);
            assert_eq!(square.piece, Some(Piece::King));
            assert_eq!(square.notation(), notation);
        };
    }

    #[test]
    fn test_square_parse() {
        validate_square!("a1", 0, 3);
        validate_square!("a2", 0, 2);
        validate_square!("a3", 0, 1);
        validate_square!("a4", 0, 0);
        validate_square!("b1", 1, 3);
        validate_square!("b2", 1, 2);
        validate_square!("b3", 1, 1);
        validate_square!("b4", 1, 0);
        validate_square!("c1", 2, 3);
        validate_square!("c2", 2, 2);
        validate_square!("c3", 2, 1);
        validate_square!("c4", 2, 0);
        validate_square!("d1", 3, 3);
        validate_square!("d2", 3, 2);
        validate_square!("d3", 3, 1);
        validate_square!("d4", 3, 0);
    }

    // Assert trying to create a SquarePair where start and end are the same is an err
    #[test]
    fn test_square_pair_try_new_same_start_end() {
        let start = Square::parse(".a1");
        let end = Square::parse(".a1");
        let result = SquarePair::try_new(start, end);
        assert!(result.is_err());
    }
}
