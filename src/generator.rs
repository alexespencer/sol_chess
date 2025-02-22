use std::fmt::Display;

use crate::{
    board::{piece::Piece, Board},
    solver::Solver,
};

use macroquad::{prelude::rand, time};

pub fn generate(num_pieces: u32, num_solutions: u32) -> GenerateStats {
    let candidate_pieces = vec![
        Piece::Pawn,
        Piece::Pawn,
        Piece::Pawn,
        Piece::Pawn,
        Piece::Bishop,
        Piece::Bishop,
        Piece::Bishop,
        Piece::Bishop,
        Piece::Knight,
        Piece::Knight,
        Piece::Knight,
        Piece::Queen,
        Piece::Rook,
        Piece::Rook,
    ];

    if num_pieces > candidate_pieces.len().try_into().unwrap() {
        panic!(
            "Number of pieces to place on the board should be <= {}",
            candidate_pieces.len()
        );
    }

    let attempts: u32 = 1000;
    let mut overall_stats = GenerateStats::new(0, 0, 0, 0., None);
    for _ in 0..attempts {
        let stats = try_generate(num_pieces, num_solutions, candidate_pieces.clone());
        overall_stats.piece_total += stats.piece_total;
        overall_stats.piece_success += stats.piece_success;
        overall_stats.total += stats.total;
        overall_stats.total_seconds += stats.total_seconds;
        overall_stats.board = stats.board;
        println!(
            "Generating puzzle.. Elapsed: {}s",
            overall_stats.total_seconds,
        );
        if overall_stats.board.is_some() {
            return overall_stats;
        }
    }

    overall_stats
}

pub struct GenerateStats {
    piece_total: u32,
    piece_success: u32,
    total: u32,
    total_seconds: f64,
    board: Option<Board>,
}

impl GenerateStats {
    fn new(
        piece_total: u32,
        piece_success: u32,
        total: u32,
        total_millis: f64,
        board: Option<Board>,
    ) -> Self {
        Self {
            piece_total,
            piece_success,
            total,
            total_seconds: total_millis,
            board,
        }
    }

    pub fn print_stats(&self) {
        let mut stats = String::new();
        add_stat(&mut stats, "Total attempts", self.total);
        add_stat(&mut stats, "Total pieces placed", self.piece_total);
        add_stat(&mut stats, "Success pieces placed", self.piece_success);
        add_stat(&mut stats, "Total time (ms)", self.total_seconds);

        println!("{}", stats);
    }

    pub fn board(self) -> Option<Board> {
        self.board
    }
}

fn add_stat<T>(stats: &mut String, name: &str, val: T)
where
    T: Display,
{
    stats.push_str(&format!("{:>30}:{:>6}\n", name, val));
}

fn try_generate(
    num_pieces: u32,
    num_solutions: u32,
    mut candidate_pieces: Vec<Piece>,
) -> GenerateStats {
    let mut board = Board::new();
    let mut piece_total = 0;
    let mut piece_success = 0;
    let now = time::get_time();
    for _ in 0..num_pieces {
        let mut placed = false;
        let empty_squares = board.empty_squares();
        let mut attempts = 15;
        while !placed {
            if attempts == 0 {
                let elapsed = time::get_time() - now;
                return GenerateStats::new(piece_total, piece_success, 1, elapsed, None);
            }

            attempts -= 1;
            piece_total += 1;

            let index = rand::gen_range(0, candidate_pieces.len());
            let piece = candidate_pieces[index];
            let square_index = rand::gen_range(0, empty_squares.len());
            let mut random_square = empty_squares[square_index].clone();
            random_square.piece = Some(piece);
            board.set(random_square.clone());
            let solutions = Solver::new(board.clone()).solve();
            if solutions.len() > 0 {
                placed = true;
                piece_success += 1;
                candidate_pieces.remove(index);
                continue;
            }

            random_square.piece = None;
            board.set(random_square);
        }
    }

    let solutions = Solver::new(board.clone()).solve();
    let elapsed = time::get_time() - now;
    if solutions.len() > num_solutions as usize {
        GenerateStats::new(piece_total, piece_success, 1, elapsed, None)
    } else {
        GenerateStats::new(piece_total, piece_success, 1, elapsed, Some(board))
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::BoardState, solver::Solver};

    use super::*;

    // Figure out a way to remove the macroquad dependencies from this package
    // #[test]
    fn generator_smoke() {
        for _ in 0..10 {
            let gen_stats = generate(5, 5);
            let board = gen_stats.board.expect("No puzzle was generated");
            assert_eq!(board.game_state, BoardState::InProgress);

            let solutions = Solver::new(board).solve();
            assert!(solutions.len() <= 5);
            assert!(solutions.len() >= 1);
        }
    }
}
