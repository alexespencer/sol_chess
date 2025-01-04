#[allow(unused)]
mod engine;

#[allow(unused)]
mod solver;

#[allow(unused)]
mod generator;

use argh::FromArgs;
use engine::board::Board;

use crate::generator::generator::generate;
use crate::solver::solver::Solver;

fn main() {
    let args: Args = argh::from_env();
    if args.generate {
        let puzzle = generate_puzzle(args.num_pieces);
        let Some(board) = puzzle else {
            return;
        };

        println!("{}", board.print());

        if args.print {
            solve_puzzle(board);
        }
    } else if let Some(board_string) = args.solve {
        let board = Board::from_string(board_string);
        let Some(board) = board else {
            println!("Invalid board string");
            return;
        };
        println!("{}", board.print());
        solve_puzzle(board);
    } else {
        println!("Use --help to see available options");
    }
}

fn solve_puzzle(board: Board) {
    let solutions = Solver::new(board).solve();
    if solutions.len() == 0 {
        println!("No solutions found");
        return;
    }
    println!("Found {} solutions", solutions.len());
    let solution = solutions.first().unwrap();
    let mut idx = 0;
    solution.iter().for_each(|m| {
        idx += 1;
        println!("{}. {}", idx, m.notation());
    });
}

fn generate_puzzle(num_pieces: Option<u32>) -> Option<Board> {
    let start = std::time::Instant::now();
    let num_pieces = num_pieces.unwrap_or(5);
    let board = generate(num_pieces);
    let elapsed = start.elapsed();

    let Some(board) = board else {
        println!(
            "Failed to generate a puzzle with {} pieces after {} ms, Try again",
            num_pieces,
            elapsed.as_millis()
        );
        return None;
    };

    println!(
        "Generated a puzzle with {} pieces after {} ms",
        num_pieces,
        elapsed.as_millis()
    );
    Some(board)
}

/// Solitaire Chess puzzle generator and solver
/// - v0.0.1 cool-mist
#[derive(FromArgs)]
struct Args {
    #[argh(switch, short = 'g')]
    /// generate a puzzle
    generate: bool,

    #[argh(option, short = 'n')]
    /// number of pieces to place on the board while generating a puzzle
    num_pieces: Option<u32>,

    #[argh(switch)]
    /// print the solution. When solving a puzzle, this is always set to true
    print: bool,

    #[argh(option, short = 's')]
    /// the board to solve
    solve: Option<String>,
}
