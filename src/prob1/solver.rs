pub trait Solver {
    fn sequence(&self) -> Box<Iterator<Item=u32>>;
    fn solve(&self) -> u32 {
        self.sequence().fold(0, |acc, item| acc + item)
    }
}
