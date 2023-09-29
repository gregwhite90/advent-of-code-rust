pub mod utils {
    use either::*;
    // Solution implements solve, parse input filename
    pub trait Solution {
        fn parse_input_file(&mut self, filename: &str);
        fn solve(&self) -> Either<i32, &str>;
    }
}