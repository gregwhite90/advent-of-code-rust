pub mod utils {
    use either::*;
    // Solution implements solve, parse input filename
    pub trait Solution {
        fn new(filename: &str) -> Self;
        fn solve(&self) -> Either<i32, &str>;
    }
}