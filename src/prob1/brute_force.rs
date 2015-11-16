use prob1::solver::Solver;

pub struct BruteForceSolver {
    upper: u32,
}

impl BruteForceSolver {
    pub fn new(upper_limit: u32) -> BruteForceSolver {
        BruteForceSolver { upper: upper_limit }
    }
}

impl Solver for BruteForceSolver {
    fn sequence(&self) -> Box<Iterator<Item=u32>> {
        Box::new((1..self.upper).filter(|n: &u32| n % 3 == 0 || n % 5 == 0))
    }
}
