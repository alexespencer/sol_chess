use crate::{
    engine::{board::Board, coord::Coord, piece::Piece, square::Square},
    solver::{self, solver::Solver},
};
use rand::{seq::*, Rng};

pub(crate) fn generate(num_pieces: u32) -> Option<Board> {
    let mut rand = rand::thread_rng();
    let candidate_pieces = vec![
        Piece::Pawn,
        Piece::Pawn,
        Piece::Pawn,
        Piece::Rook,
        Piece::Bishop,
        Piece::Knight,
        Piece::Knight,
        Piece::King,
        Piece::Queen,
    ];
    let attempts = 1000;
    for i in 0..attempts {
        let board = try_generate(num_pieces, candidate_pieces.clone(), rand.clone());
        if let Some(board) = board {
            return Some(board);
        }
    }

    None
}

fn try_generate(
    num_pieces: u32,
    mut candidate_pieces: Vec<Piece>,
    mut rand: rand::prelude::ThreadRng,
) -> Option<Board> {
    let mut board = Board::new();
    for _ in 0..num_pieces {
        let mut placed = false;
        let empty_squares = board.empty_squares();
        let mut attempts = 15;
        while !placed {
            if attempts == 0 {
                return None;
            }

            attempts -= 1;

            let index = rand.gen_range(0..candidate_pieces.len());
            let piece = candidate_pieces[index];
            let coord = empty_squares.choose(&mut rand).unwrap().clone();

            board.set(Square::Occupied(piece.clone(), coord.clone()));
            let solutions = Solver::new(board.clone()).solve();
            if solutions.len() > 0 {
                placed = true;
                candidate_pieces.remove(index);
                continue;
            }
            board.set(Square::Empty(coord));
        }
    }

    let solutions = Solver::new(board.clone()).solve();
    if solutions.len() > 1 {
        None
    } else {
        Some(board)
    }
}

#[cfg(test)]
mod tests {
    use crate::{engine::board::GameState, solver::solver::Solver};

    use super::*;

    #[test]
    fn generator_smoke() {
        let board = generate().unwrap();
        assert_eq!(board.game_state, GameState::InProgress);

        let solutions = Solver::new(board).solve();
        assert_ne!(solutions.len(), 0);
    }
}
