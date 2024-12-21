pub mod utils;
pub mod year24;

pub trait Solver<T> {
    fn parse(input: &str) -> T;
    fn part_1(input: T) -> i32;
    fn part_2(input: T) -> i32;
}
