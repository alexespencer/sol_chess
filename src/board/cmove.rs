use super::{piece::Piece, square::Square};
use eyre::{Result, ensure};

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
pub struct CMove {
    from: Square,
    to: Square,
}

impl CMove {
    // TODO: moves could be created and validated from a Board?

    pub fn try_new(from: Square, to: Square) -> Result<Self> {
        ensure!(
            from.location() != to.location(),
            "from/to location must not be the same"
        );
        Ok(CMove { from, to })
    }

    pub fn notation(&self) -> String {
        let piece_qualifier = match &self.from.piece() {
            Piece::Pawn => self.from.location().file_notation(),
            p => p.to_string(),
        };
        format!("{}x{}", piece_qualifier, self.to.notation())
    }

    pub fn from(&self) -> &Square {
        &self.from
    }

    pub fn to(&self) -> &Square {
        &self.to
    }

    pub fn dx(&self) -> isize {
        self.to.location().file() as isize - self.from.location().file() as isize
    }

    pub fn dy(&self) -> isize {
        self.to.location().rank() as isize - self.from.location().rank() as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_pair_try_new_same_start_end() {
        let start = Square::parse("Ka1").unwrap();
        let end = Square::parse("Ka1").unwrap();
        let result = CMove::try_new(start, end);
        assert!(result.is_err());
    }
}
