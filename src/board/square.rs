use super::constants::BOARD_SIZE;
use super::piece::Piece;
use core::fmt;
use eyre::{Result, ensure};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Square {
    file: u8,
    rank: u8,
    piece: Option<Piece>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SquarePair {
    start: Square,
    end: Square,
}

impl Square {
    pub fn file(&self) -> u8 {
        self.file
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }

    pub fn piece(&self) -> Option<Piece> {
        self.piece
    }

    pub fn new(file: u8, rank: u8, piece: Option<Piece>) -> Self {
        Square { file, rank, piece }
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
        Square::new(file, rank, piece)
    }

    pub fn file_notation(&self) -> String {
        String::from("abcd".chars().nth(self.file() as usize).unwrap())
    }

    pub fn rank_notation(&self) -> String {
        format!("{}", BOARD_SIZE - self.rank)
    }

    pub fn notation(&self) -> String {
        format!(
            "{}{}{}",
            self.piece_notation(),
            self.file_notation(),
            BOARD_SIZE - self.rank
        )
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
        self.end.file as isize - self.start.file as isize
    }

    pub fn dy(&self) -> isize {
        self.end.rank as isize - self.start.rank as isize
    }

    pub fn try_new(start: Square, end: Square) -> Result<Self> {
        ensure!(
            start.file != end.file || start.rank != end.rank,
            "start position must be different to end position"
        );
        Ok(SquarePair { start, end })
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({},{})", self.notation(), self.file, self.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! validate_square {
        ($notation:literal, $file:expr, $rank:expr) => {
            let notation = format!("{}{}", "K", $notation);
            let square = Square::parse(&notation);
            assert_eq!(square.file, $file);
            assert_eq!(square.rank, $rank);
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
