mod horspool;
mod memchr;
mod naive_search;
mod pattern;

pub use horspool::*;
pub use memchr::*;
pub use naive_search::*;
pub use pattern::*;

pub trait Searcher<'a>: Iterator<Item = usize> {
    fn new(pattern: &'a Pattern, data: &'a [u8]) -> Self;
}
