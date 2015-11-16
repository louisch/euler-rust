mod solver;
mod brute_force;

use prob1::solver::Solver;
use prob1::brute_force::BruteForceSolver;

pub fn demo(upper_limit: u32) {
    println!("Brute Force answer: {}", BruteForceSolver::new(upper_limit).solve());
}
