#[allow(unused)]
mod engine;

#[allow(unused)]
mod solver;

#[allow(unused)]
mod generator;

use crate::generator::generator::generate;
use crate::solver::solver::Solver;

fn main() {
    let start = std::time::Instant::now();
    let board = generate();
    let elapsed = start.elapsed();

    println!("Generated a problem in {} ms", elapsed.as_millis());

    let Some(board) = board else {
        println!(
            "Failed to generate a board after {} ms, Try again",
            elapsed.as_millis()
        );
        return;
    };

    println!("{}", board.print());
    let solutions = Solver::new(board).solve();
    println!("Found {} solutions", solutions.len());
    let solution = solutions.first().unwrap();
    let mut idx = 0;
    solution.iter().for_each(|m| {
        idx += 1;
        println!("{}. {}", idx, m.notation());
    });
}
