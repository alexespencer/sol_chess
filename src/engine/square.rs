use crate::engine::constants::BOARD_SIZE;

use super::piece::Piece;
use core::fmt;

#[derive(Clone, Eq, Hash, PartialEq)]
pub(crate) struct Square {
    // a = 0, b = 1, c = 2, d = 3 and so on.
    pub(crate) file: usize,

    // 1 = 0, 2 = 1, 3 = 2, 4 = 3 and so on.
    pub(crate) rank: usize,

    pub(crate) piece: Option<Piece>,
}

pub(crate) struct SquarePair {
    pub(crate) start: Square,
    pub(crate) end: Square,
    pub(crate) dx: usize,
    pub(crate) dy: usize,
    pub(crate) x_dir: i8,
    pub(crate) y_dir: i8,
}

impl Square {
    pub(crate) fn new(file: usize, rank: usize, piece: Option<Piece>) -> Self {
        Square { file, rank, piece }
    }

    pub(crate) fn parse(notation: &str) -> Self {
        let mut chars = notation.chars();
        let piece = chars.next().expect("Piece missing");
        let piece = Piece::parse(&piece.to_string());
        let file = chars.next().expect("File missing");
        let file = match file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            _ => panic!("file should be between a-d"),
        };

        let rank = chars.next().unwrap().to_digit(10).expect("rank missing") as usize;
        if rank < 1 || rank > BOARD_SIZE {
            panic!("rank should be between 1-{}", BOARD_SIZE);
        }
        let rank = BOARD_SIZE - rank;
        Square::new(file, rank, piece)
    }

    pub(crate) fn file_notation(&self) -> String {
        String::from("abcd".chars().nth(self.file).unwrap())
    }

    pub(crate) fn rank_notation(&self) -> String {
        format!("{}", BOARD_SIZE - self.rank)
    }

    pub(crate) fn notation(&self) -> String {
        format!(
            "{}{}{}",
            self.piece_notation(),
            self.file_notation(),
            BOARD_SIZE - self.rank
        )
    }

    pub(crate) fn is_occupied(&self) -> bool {
        self.piece.is_some()
    }

    fn piece_notation(&self) -> String {
        if self.piece.is_none() {
            "".to_string()
        } else {
            self.piece.unwrap().notation()
        }
    }
}

impl SquarePair {
    pub(crate) fn new(start: Square, end: Square) -> Self {
        let mut dx = 0;
        let mut dy = 0;
        let mut x_dir = 0;
        let mut y_dir = 0;
        if start.file > end.file {
            x_dir = -1;
            dx = start.file - end.file;
        } else if end.file > start.file {
            x_dir = 1;
            dx = end.file - start.file;
        }

        if start.rank > end.rank {
            y_dir = -1;
            dy = start.rank - end.rank;
        } else if end.rank > start.rank {
            y_dir = 1;
            dy = end.rank - start.rank;
        }

        SquarePair {
            start,
            end,
            dx,
            dy,
            x_dir,
            y_dir,
        }
    }

    pub(crate) fn is_different(&self) -> bool {
        self.dx != 0 || self.dy != 0
    }
}

macro_rules! sq {
    ($sq:literal) => {
        Square::parse($sq)
    };
}

pub(crate) use sq;

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
}
