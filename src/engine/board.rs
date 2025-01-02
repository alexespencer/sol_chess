use super::{
    coord::{at, Coord},
    piece::Piece,
    square::Square,
};

pub(crate) struct Board {
    cells: [[Square; 4]; 4],
}

pub(crate) struct Move {
    piece: Piece,
    from: Coord,
    to: Coord,
    target: Piece,
}

impl Board {
    pub(crate) fn new() -> Self {
        Board {
            cells: [
                [
                    Square::Empty(at!("a4")),
                    Square::Empty(at!("b4")),
                    Square::Empty(at!("c4")),
                    Square::Empty(at!("d4")),
                ],
                [
                    Square::Empty(at!("a3")),
                    Square::Empty(at!("b3")),
                    Square::Empty(at!("c3")),
                    Square::Empty(at!("d3")),
                ],
                [
                    Square::Empty(at!("a2")),
                    Square::Empty(at!("b2")),
                    Square::Empty(at!("c2")),
                    Square::Empty(at!("d2")),
                ],
                [
                    Square::Empty(at!("a1")),
                    Square::Empty(at!("b1")),
                    Square::Empty(at!("c1")),
                    Square::Empty(at!("d1")),
                ],
            ],
        }
    }

    pub(crate) fn place(&mut self, square: Square) -> Square {
        let coord = square.coord();
        std::mem::replace(&mut self.cells[coord.file][coord.rank], square)
    }

    pub(crate) fn legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();
        for file in 0..4 {
            for rank in 0..4 {
                if let Square::Occupied(piece, _) = self.cells[file][rank] {
                    let mut moves = match piece {
                        Piece::King => self.king_legal_moves(Coord::new(file, rank)),
                        _ => Vec::with_capacity(0),
                    };

                    legal_moves.append(&mut moves);
                }
            }
        }
        legal_moves
    }

    fn king_legal_moves(&self, start: Coord) -> Vec<Move> {
        let mut candidates = Vec::new();
        let x = start.file;
        let y = start.rank;
        for file in (x.saturating_sub(1))..(x.saturating_add(2)) {
            for rank in (y.saturating_sub(1))..(y.saturating_add(2)) {
                if file == x && rank == y {
                    continue;
                }
                let target = &self.cells[file][rank];
                if let Square::Occupied(piece, _) = target {
                    candidates.push(Move {
                        piece: Piece::King,
                        from: Coord::new(x, y),
                        to: Coord::new(file, rank),
                        target: *piece,
                    });
                }
            }
        }

        candidates
    }

    pub(crate) fn print(&self) -> String {
        let mut builder: Vec<char> = Vec::new();
        for rank in 0..4 {
            for file in 0..4 {
                match self.cells[file][rank] {
                    Square::Empty(_) => builder.push('.'),
                    Square::Occupied(Piece::King, _) => builder.push('K'),
                    Square::Occupied(Piece::Queen, _) => builder.push('Q'),
                    Square::Occupied(Piece::Bishop, _) => builder.push('B'),
                    Square::Occupied(Piece::Knight, _) => builder.push('N'),
                    Square::Occupied(Piece::Rook, _) => builder.push('R'),
                    Square::Occupied(Piece::Pawn, _) => builder.push('P'),
                }
            }
            builder.push('\n');
        }

        builder.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::piece::p;
    use crate::engine::square::sq;

    use super::*;

    macro_rules! validate_board {
        ($board:expr, $row1:literal, $row2:literal, $row3:literal, $row4:literal) => {
            let printed = $board.print();
            assert_eq!(
                printed,
                format!("{}\n{}\n{}\n{}\n", $row1, $row2, $row3, $row4)
            );
        };
    }

    #[test]
    fn test_board_place() {
        let mut board = Board::new();
        assert!(board.place(sq!("K", "a1")).is_empty());
        assert!(board.place(sq!("Q", "a2")).is_empty());
        assert!(board.place(sq!("B", "c3")).is_empty());
        assert!(board.place(sq!("N", "c4")).is_empty());
        assert!(board.place(sq!("R", "d1")).is_empty());
        assert!(board.place(sq!("P", "d4")).is_empty());
        assert!(board.place(sq!("N", "b2")).is_empty());
        let existing = board.place(sq!("P", "c4"));
        assert!(existing.occupied().is_some());
        assert_eq!(Piece::Knight, existing.occupied().unwrap());
        validate_board!(board, "..PP", "..B.", "QN..", "K..R");
    }

    #[test]
    fn test_legal_moves_king_corner() {
        let mut board = Board::new();
        board.place(sq!("K", "a2"));
        board.place(sq!("P", "a1"));
        board.place(sq!("P", "c4"));

        let legal_moves = board.legal_moves();
        assert_eq!(legal_moves.len(), 1);

        board.place(sq!("P", "b1"));
        let legal_moves = board.legal_moves();
        assert_eq!(legal_moves.len(), 2);
    }

    #[test]
    fn test_legal_moves_king_center() {
        let mut board = Board::new();
        board.place(sq!("K", "c3"));
        board.place(sq!("P", "a1"));
        board.place(sq!("P", "c4"));
        board.place(sq!("P", "b2"));
        board.place(sq!("P", "b3"));

        let legal_moves = board.legal_moves();
        assert_eq!(legal_moves.len(), 3);
    }
}
