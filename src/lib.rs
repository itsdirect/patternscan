mod horspool;
mod naive_search;
mod pattern;

pub use horspool::*;
pub use naive_search::*;
pub use pattern::*;

pub trait Searcher<'a>: Iterator<Item = usize> {
    fn new(pattern: &'a Pattern, data: &'a [u8]) -> Self;
}
