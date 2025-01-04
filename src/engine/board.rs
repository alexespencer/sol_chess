use std::collections::HashSet;

use super::{
    coord::{at, Coord},
    piece::Piece,
    r#move::Move,
    square::{sq, Square},
};

pub(crate) struct Board {
    pub(crate) cells: [[Square; 4]; 4],
    legal_moves: HashSet<Move>,
    pieces_remaining: i8,
    game_state: GameState,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GameState {
    NotStarted,
    InProgress,
    Lost,
    Won,
}

impl Board {
    pub(crate) fn new() -> Self {
        Board {
            cells: [
                [sq!("a4"), sq!("b4"), sq!("c4"), sq!("d4")],
                [sq!("a3"), sq!("b3"), sq!("c3"), sq!("d3")],
                [sq!("a2"), sq!("b2"), sq!("c2"), sq!("d2")],
                [sq!("a1"), sq!("b1"), sq!("c1"), sq!("d1")],
            ],
            legal_moves: HashSet::new(),
            pieces_remaining: 0,
            game_state: GameState::NotStarted,
        }
    }

    pub(crate) fn set(&mut self, square: Square) -> Square {
        let coord = square.coord();
        let new_is_occuppied = square.occupied().is_some();
        let existing = std::mem::replace(&mut self.cells[coord.file][coord.rank], square);

        // If placing a piece on a blank, increment piece count
        if existing.is_empty() && new_is_occuppied {
            self.pieces_remaining += 1;
        }

        // If placing a blank on a piece, decrement piece count
        if existing.occupied().is_some() && !new_is_occuppied {
            self.pieces_remaining -= 1;
        }

        self.calc_legal_moves();
        self.calc_game_state();
        existing
    }

    pub(crate) fn make_move(&mut self, mv: Move) -> Option<Move> {
        if self.legal_moves.contains(&mv) {
            // Remove from source
            let source = std::mem::replace(
                &mut self.cells[mv.from.coord_ref().file][mv.from.coord_ref().rank],
                Square::Empty(mv.from.coord()),
            );

            let target = Square::Occupied(source.occupied().unwrap(), mv.to.coord());

            // Place it on target
            std::mem::replace(
                &mut self.cells[mv.to.coord_ref().file][mv.to.coord_ref().rank],
                target,
            );

            self.pieces_remaining -= 1;
            self.calc_legal_moves();
            self.calc_game_state();
            Some(mv)
        } else {
            println!("Invalid move - {}", mv.notation());
            println!("Legal moves - ");
            for m in &self.legal_moves {
                println!("{}", m.notation());
            }
            None
        }
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

    fn calc_legal_moves(&mut self) {
        self.legal_moves.clear();
        for file in 0..4 {
            for rank in 0..4 {
                if let Square::Occupied(piece, _) = self.cells[file][rank] {
                    let source = &self.cells[file][rank];
                    let mut moves = match piece {
                        Piece::King => self.king_legal_moves(source),
                        Piece::Pawn => self.pawn_legal_moves(source),
                        Piece::Knight => self.knight_legal_moves(source),
                        Piece::Bishop => self.bishop_legal_moves(source),
                        Piece::Rook => self.rook_legal_moves(source),
                        Piece::Queen => self.queen_legal_moves(source),
                    };

                    moves.into_iter().for_each(|m| {
                        self.legal_moves.insert(m);
                    });
                }
            }
        }
    }

    fn calc_game_state(&mut self) {
        self.game_state = if self.pieces_remaining == 0 {
            GameState::NotStarted
        } else if self.pieces_remaining == 1 {
            GameState::Won
        } else if self.legal_moves.is_empty() {
            GameState::Lost
        } else {
            GameState::InProgress
        }
    }

    fn king_legal_moves(&self, start: &Square) -> Vec<Move> {
        self.rect(start.coord(), 1)
            .into_iter()
            .map(|s| Board::create_move(start, s))
            .collect()
    }

    fn pawn_legal_moves(&self, start: &Square) -> Vec<Move> {
        self.rect(start.coord(), 1)
            .into_iter()
            .filter(|target| {
                target.coord_ref().rank < start.coord_ref().rank
                    && target.coord_ref().file != start.coord_ref().file
            })
            .map(|s| Board::create_move(start, s))
            .collect()
    }

    fn knight_legal_moves(&self, start: &Square) -> Vec<Move> {
        self.rect(start.coord(), 2)
            .into_iter()
            .filter(|target| {
                let dx = (start.coord_ref().file as isize - target.coord_ref().file as isize).abs();
                let dy = (start.coord_ref().rank as isize - target.coord_ref().rank as isize).abs();
                (dx == 1 && dy == 2) || (dx == 2 && dy == 1)
            })
            .map(|s| Board::create_move(start, s))
            .collect()
    }

    fn bishop_legal_moves(&self, start: &Square) -> Vec<Move> {
        self.diag(start.coord())
            .into_iter()
            .map(|s| Board::create_move(start, s))
            .collect()
    }

    fn rook_legal_moves(&self, start: &Square) -> Vec<Move> {
        self.line(start.coord())
            .into_iter()
            .map(|s| Board::create_move(start, s))
            .collect()
    }

    fn queen_legal_moves(&self, start: &Square) -> Vec<Move> {
        let line = self.line(start.coord()).into_iter();
        let diag = self.diag(start.coord()).into_iter();
        line.chain(diag)
            .map(|s| Board::create_move(start, s))
            .collect()
    }

    fn rect(&self, start: Coord, radius: usize) -> Vec<Square> {
        let mut range = Vec::new();
        let x_min = start.file.saturating_sub(radius);
        let y_min = start.rank.saturating_sub(radius);
        let x_max = start.file + radius + 1;
        let y_max = start.rank + radius + 1;

        for file in (x_min)..(x_max) {
            for rank in (y_min)..(y_max) {
                if file > 3 || rank > 3 {
                    continue;
                }

                if (file, rank) == (start.file, start.rank) {
                    continue;
                }

                if self.cells[file][rank].occupied().is_none() {
                    continue;
                }

                range.push(self.cells[file][rank].clone());
            }
        }
        range
    }

    fn diag(&self, start: Coord) -> Vec<Square> {
        let mut range = Vec::new();

        // North West
        if (start.rank > 0 && start.file > 0) {
            let mut north = start.rank;
            let mut west = start.file;
            while north != 0 && west != 0 {
                north -= 1;
                west -= 1;
                if let Some(piece) = self.cells[west][north].occupied() {
                    range.push(self.cells[west][north].clone());
                    break;
                }
            }
        }

        // North East
        if (start.rank > 0 && start.file < 3) {
            let mut north = start.rank;
            let mut east = start.file;
            while north != 0 && east < 3 {
                north -= 1;
                east += 1;
                if let Some(piece) = self.cells[east][north].occupied() {
                    range.push(self.cells[east][north].clone());
                    break;
                }
            }
        }

        // South West
        if (start.rank < 3 && start.file > 0) {
            let mut south = start.rank;
            let mut west = start.file;
            while south < 4 && west != 0 {
                south += 1;
                west -= 1;
                if let Some(piece) = self.cells[west][south].occupied() {
                    range.push(self.cells[west][south].clone());
                    break;
                }
            }
        }

        // South East
        if (start.rank < 3 && start.file < 3) {
            let mut south = start.rank;
            let mut east = start.file;
            while south < 3 && east < 3 {
                south += 1;
                east += 1;
                if let Some(piece) = self.cells[east][south].occupied() {
                    range.push(self.cells[east][south].clone());
                    break;
                }
            }
        }

        range
    }

    fn line(&self, start: Coord) -> Vec<Square> {
        let mut range = Vec::new();

        if (start.rank > 0) {
            // North
            let mut north = start.rank;
            while north != 0 {
                north -= 1;
                if let Some(piece) = self.cells[start.file][north].occupied() {
                    range.push(self.cells[start.file][north].clone());
                    break;
                }
            }
        }

        if (start.rank < 3) {
            // South
            let mut south = start.rank;
            while south < 3 {
                south += 1;
                if let Some(piece) = self.cells[start.file][south].occupied() {
                    range.push(self.cells[start.file][south].clone());
                    break;
                }
            }
        }

        if (start.file > 0) {
            //West
            let mut west = start.file;
            while west != 0 {
                west -= 1;
                if let Some(piece) = self.cells[west][start.rank].occupied() {
                    range.push(self.cells[west][start.rank].clone());
                    break;
                }
            }
        }

        if (start.file < 3) {
            // East
            let mut east = start.file;
            while east < 3 {
                east += 1;
                if let Some(piece) = self.cells[east][start.rank].occupied() {
                    range.push(self.cells[east][start.rank].clone());
                    break;
                }
            }
        }

        range
    }

    fn create_move(start: &Square, target: Square) -> Move {
        Move::new(start.clone(), target)
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::piece::p;
    use crate::engine::r#move::mv;
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

    macro_rules! validate_legal_moves {
        ($board:expr, $($move:expr,)*) => {
            let mut legal_moves = $board.legal_moves.iter().map(|m| m.clone()).collect::<Vec<Move>>();

            $(
                assert!(legal_moves.contains(&$move));
                let position = legal_moves.iter().position(|m| m == &$move).unwrap();
                legal_moves.remove(position);
            )*

            if (legal_moves.len() > 0) {
                println!("The following moves were not matched - ");
                for m in &legal_moves {
                    println!("{}", m.notation());
                }

                assert!(false);
            }
        };
    }

    #[test]
    fn test_board_place() {
        let mut board = Board::new();
        assert!(board.set(sq!("K", "a1")).is_empty());
        assert!(board.set(sq!("Q", "a2")).is_empty());
        assert!(board.set(sq!("B", "c3")).is_empty());
        assert!(board.set(sq!("N", "c4")).is_empty());
        assert!(board.set(sq!("R", "d1")).is_empty());
        assert!(board.set(sq!("P", "d4")).is_empty());
        assert!(board.set(sq!("N", "b2")).is_empty());
        let existing = board.set(sq!("P", "c4"));
        assert!(existing.occupied().is_some());
        assert_eq!(Piece::Knight, existing.occupied().unwrap());
        validate_board!(board, "..PP", "..B.", "QN..", "K..R");
    }

    #[test]
    fn test_legal_moves() {
        let mut board = Board::new();
        assert_eq!(0, board.pieces_remaining);
        assert_eq!(0, board.legal_moves.len());
        assert!(board.make_move(mv!("R", "b2", "d1", "N")).is_none());

        board.set(sq!("Q", "a4"));
        board.set(sq!("K", "a2"));
        board.set(sq!("P", "a1"));
        board.set(sq!("P", "b3"));
        board.set(sq!("R", "b2"));
        board.set(sq!("P", "c4"));
        board.set(sq!("K", "c3"));
        board.set(sq!("B", "c1"));
        board.set(sq!("B", "d2"));
        board.set(sq!("N", "d1"));

        assert_eq!(10, board.pieces_remaining);

        // Q . P .
        // . P K .
        // K R . B
        // P . B N
        validate_legal_moves!(
            board,
            mv!("K", "a2", "a1", "P"),
            mv!("K", "a2", "b2", "R"),
            mv!("K", "a2", "b3", "P"),
            mv!("K", "c3", "b2", "R"),
            mv!("K", "c3", "b3", "P"),
            mv!("K", "c3", "c4", "P"),
            mv!("K", "c3", "d2", "B"),
            mv!("P", "a1", "b2", "R"),
            mv!("P", "b3", "c4", "P"),
            mv!("P", "b3", "a4", "Q"),
            mv!("Q", "a4", "a2", "K"),
            mv!("Q", "a4", "b3", "P"),
            mv!("Q", "a4", "c4", "P"),
            mv!("R", "b2", "a2", "K"),
            mv!("R", "b2", "b3", "P"),
            mv!("R", "b2", "d2", "B"),
            mv!("B", "c1", "b2", "R"),
            mv!("B", "c1", "d2", "B"),
            mv!("B", "d2", "c3", "K"),
            mv!("B", "d2", "c1", "B"),
            mv!("N", "d1", "b2", "R"),
            mv!("N", "d1", "c3", "K"),
        );

        assert_eq!(10, board.pieces_remaining);

        // Validate some illegal moves
        assert!(board.make_move(mv!("K", "a2", "a2", "P")).is_none());
        assert!(board.make_move(mv!("R", "b2", "d1", "N")).is_none());

        board.set(sq!("b2"));
        board.set(sq!("c4"));
        board.set(sq!("R", "c1"));

        // Q . . .
        // . P K .
        // K . . B
        // P . R N
        validate_legal_moves!(
            board,
            mv!("K", "a2", "a1", "P"),
            mv!("K", "a2", "b3", "P"),
            mv!("K", "c3", "b3", "P"),
            mv!("K", "c3", "d2", "B"),
            mv!("P", "b3", "a4", "Q"),
            mv!("B", "d2", "c3", "K"),
            mv!("B", "d2", "c1", "R"),
            mv!("Q", "a4", "a2", "K"),
            mv!("Q", "a4", "b3", "P"),
            mv!("R", "c1", "a1", "P"),
            mv!("R", "c1", "c3", "K"),
            mv!("R", "c1", "d1", "N"),
            mv!("N", "d1", "c3", "K"),
        );

        assert_eq!(8, board.pieces_remaining);
    }

    #[test]
    fn test_smoke_puzzle() {
        let mut board = Board::new();
        assert_eq!(GameState::NotStarted, board.game_state);
        assert_eq!(0, board.pieces_remaining);

        // K . . .
        // . P . .
        // . . R .
        // N . . .
        board.set(sq!("K", "a4"));
        assert_eq!(GameState::Won, board.game_state);

        board.set(sq!("P", "b3"));
        board.set(sq!("R", "c2"));
        board.set(sq!("N", "a1"));

        assert_eq!(GameState::InProgress, board.game_state);
        assert_eq!(4, board.pieces_remaining);

        assert!(board.make_move(mv!("N", "a1", "c2", "R")).is_some());
        assert_eq!(3, board.pieces_remaining);
        assert_eq!(GameState::InProgress, board.game_state);

        assert!(board.make_move(mv!("P", "b3", "a4", "K")).is_some());
        assert_eq!(2, board.pieces_remaining);
        assert_eq!(GameState::Lost, board.game_state);

        // P . . .
        // . . . .
        // . . N .
        // . . . .

        board.set(sq!("P", "a1"));
        board.set(sq!("Q", "a3"));

        // P . . .
        // Q . . .
        // . . N .
        // P . . .
        assert_eq!(4, board.pieces_remaining);
        assert_eq!(GameState::InProgress, board.game_state);

        board.make_move(mv!("Q", "a3", "a4", "P"));
        board.make_move(mv!("N", "c2", "a1", "P"));
        assert_eq!(2, board.pieces_remaining);
        assert_eq!(GameState::InProgress, board.game_state);

        // Q . . .
        // . . . .
        // . . . .
        // N . . .
        board.make_move(mv!("Q", "a4", "a1", "N"));
        assert_eq!(1, board.pieces_remaining);
        assert_eq!(GameState::Won, board.game_state);
    }
}
