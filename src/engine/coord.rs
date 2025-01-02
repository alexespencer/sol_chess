use core::fmt;

#[derive(Clone, PartialEq)]
pub(crate) struct Coord {
    // a = 0, b = 1, c = 2, d = 3
    pub(crate) file: usize,

    // 1 = 0, 2 = 1, 3 = 2, 4 = 3
    pub(crate) rank: usize,
    pub(crate) notation: String,
}

impl Coord {
    pub(crate) fn parse(coord: &str) -> Self {
        let mut chars = coord.chars();
        let file = chars.next().expect("file missing");
        let file = match file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            _ => panic!("file should be between a-d"),
        };

        let rank = chars.next().unwrap().to_digit(10).expect("rank missing") as usize;
        if rank < 1 || rank > 4 {
            panic!("rank should be between 1-4");
        }
        let rank = 4 - rank;

        Coord {
            file,
            rank,
            notation: coord.to_string(),
        }
    }

    pub(crate) fn new(file: usize, rank: usize) -> Self {
        Coord {
            file,
            rank,
            notation: Self::get_notation(file, rank),
        }
    }

    fn get_notation(rank: usize, file: usize) -> String {
        format!("{}{}", "abcd".chars().nth(file).unwrap(), 4 - rank)
    }
}

macro_rules! at {
    ($coord:literal) => {
        Coord::parse($coord)
    };
}

pub(crate) use at;

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({},{})", self.notation, self.file, self.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! validate_coord {
        ($notation:literal, $file:expr, $rank:expr) => {
            let coord = at!($notation);
            assert_eq!(coord.file, $file);
            assert_eq!(coord.rank, $rank);
            assert_eq!(coord.notation, $notation);
        };
    }

    #[test]
    fn test_coord_parse() {
        validate_coord!("a1", 0, 3);
        validate_coord!("a2", 0, 2);
        validate_coord!("a3", 0, 1);
        validate_coord!("a4", 0, 0);
        validate_coord!("b1", 1, 3);
        validate_coord!("b2", 1, 2);
        validate_coord!("b3", 1, 1);
        validate_coord!("b4", 1, 0);
        validate_coord!("c1", 2, 3);
        validate_coord!("c2", 2, 2);
        validate_coord!("c3", 2, 1);
        validate_coord!("c4", 2, 0);
        validate_coord!("d1", 3, 3);
        validate_coord!("d2", 3, 2);
        validate_coord!("d3", 3, 1);
        validate_coord!("d4", 3, 0);
    }
}
