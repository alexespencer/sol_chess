use crate::board::constants::FILE_CHARS;

use super::constants::BOARD_SIZE;
use super::piece::Piece;
use eyre::{Context, OptionExt, Result, bail, ensure};

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

    pub fn try_new(file: u8, rank: u8) -> Result<Self> {
        ensure!(
            file < BOARD_SIZE,
            "file should be between 0-{}",
            BOARD_SIZE - 1
        );
        ensure!(
            rank < BOARD_SIZE,
            "rank should be between 0-{}",
            BOARD_SIZE - 1
        );
        Ok(Location { file, rank })
    }

    pub fn file_notation(&self) -> String {
        String::from(
            FILE_CHARS
                .chars()
                .nth(self.file() as usize)
                .expect("checked on construction"),
        )
    }

    pub fn rank_notation(&self) -> String {
        format!("{}", BOARD_SIZE - self.rank)
    }

    pub fn notation(&self) -> String {
        format!("{}{}", self.file_notation(), self.rank_notation())
    }

    pub fn try_parse(notation: &str) -> Result<Self> {
        ensure!(notation.len() == 2, "notation for Location is 2 chars");

        let file = notation.chars().nth(0).expect("File missing");
        let file = match file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            _ => panic!("file should be between a-d"),
        };

        let rank = notation
            .chars()
            .nth(1)
            .ok_or_eyre("no rank")
            .context("getting rank")?
            .to_digit(10)
            .ok_or_eyre("rank was not digit")
            .context("parse rank digit")? as u8;

        ensure!(
            (1..=BOARD_SIZE).contains(&rank),
            format!("rank should be between 1-{}", BOARD_SIZE)
        );
        let rank = BOARD_SIZE - rank;
        Location::try_new(file, rank)
    }
}

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct OccupiedSquare {
    location: Location,
    piece: Piece,
}

impl OccupiedSquare {
    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn new(location: Location, piece: Piece) -> Self {
        OccupiedSquare { location, piece }
    }

    pub fn parse(notation: &str) -> Result<Self> {
        let mut chars = notation.chars();
        let piece = chars.next().expect("Piece missing");
        let piece = match piece {
            '.' => bail!("no longer tracking empty squares"),
            c => Piece::try_from(c.to_string().as_str()).context("parse char to Piece")?,
        };
        let location = Location::try_parse(chars.as_str());
        Ok(OccupiedSquare::new(
            location.context("parse location")?,
            piece,
        ))
    }

    pub fn notation(&self) -> String {
        format!("{}{}", self.piece_notation(), self.location().notation(),)
    }

    fn piece_notation(&self) -> String {
        self.piece.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! validate_square {
        ($notation:literal, $file:expr, $rank:expr) => {
            let notation = format!("{}{}", "K", $notation);
            let square = OccupiedSquare::parse(&notation).unwrap();
            assert_eq!(square.location().file(), $file);
            assert_eq!(square.location().rank(), $rank);
            assert_eq!(square.piece, Piece::King);
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
    fn test_location_parse() {
        let location = Location::try_parse("a1").unwrap();
        assert_eq!(location.file, 0);
        assert_eq!(location.rank, 3);
    }
}
