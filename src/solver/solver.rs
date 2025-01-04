use crate::engine::{
    board::{Board, GameState},
    r#move::Move,
};

pub(crate) struct Solver {
    pub(crate) board: Board,
    moves: Vec<Move>,
}

impl Solver {
    pub(crate) fn new(board: Board) -> Solver {
        Solver {
            board,
            moves: vec![],
        }
    }

    fn clone(&self, m: Move) -> Self {
        let mut moves = self.moves.clone();
        let mut board = self.board.clone();
        moves.push(m.clone());
        board.make_move(m);
        Solver { board, moves }
    }

    pub(crate) fn solve(&self) -> Vec<Vec<Move>> {
        let mut solutions = Vec::new();
        if let GameState::Won = self.board.game_state {
            solutions.push(self.moves.clone());
            return solutions;
        }

        let GameState::InProgress = self.board.game_state else {
            return solutions;
        };

        self.board.legal_moves.iter().for_each(|m| {
            let mut solver = self.clone(m.clone());
            let more_solutions = solver.solve();
            solutions.extend(more_solutions);
        });

        solutions
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::{
        coord::{at, Coord},
        piece::{p, Piece},
        square::{sq, Square},
    };

    use super::*;

    #[test]
    fn solver_smoke() {
        let mut board = Board::new();
        // . R . .
        // R . . P
        // B . B N
        // P . N .

        board.set(sq!("P", "a1"));
        board.set(sq!("B", "a2"));
        board.set(sq!("R", "a3"));
        board.set(sq!("R", "b4"));
        board.set(sq!("N", "c1"));
        board.set(sq!("B", "c2"));
        board.set(sq!("N", "d2"));
        board.set(sq!("P", "d3"));

        let solver = Solver::new(board.clone());
        let solutions = solver.solve();

        assert_eq!(10, solutions.len());

        for solution in solutions {
            let mut board = board.clone();
            solution.iter().for_each(|m| {
                board.make_move(m.clone());
            });
            assert_eq!(GameState::Won, board.game_state);
        }
    }

    #[test]
    fn solver_smoke_no_solution() {
        // . R . .
        // R . . .
        // B . B N
        // P . N .

        let mut board = Board::new();
        board.set(sq!("P", "a1"));
        board.set(sq!("B", "a2"));
        board.set(sq!("R", "a3"));
        board.set(sq!("R", "b4"));
        board.set(sq!("N", "c1"));
        board.set(sq!("B", "c2"));
        board.set(sq!("N", "d2"));

        let solver = Solver::new(board.clone());
        let solutions = solver.solve();

        assert_eq!(0, solutions.len());
    }
}
