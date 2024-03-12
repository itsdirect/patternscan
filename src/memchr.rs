use crate::pattern::Pattern;
use crate::Searcher;

pub struct Memchr<'a> {
    pattern: &'a Pattern,
    prefix: usize,
    first: u8,
    data: &'a [u8],
    index: usize,
}

impl<'a> Searcher<'a> for Memchr<'a> {
    fn new(pattern: &'a Pattern, data: &'a [u8]) -> Self {
        let (prefix, first) = pattern
            .0
            .iter()
            .enumerate()
            .find_map(|(i, b)| b.0.map(|b| (i, b)))
            .unwrap();

        Self {
            pattern,
            prefix,
            first,
            data,
            index: 0,
        }
    }
}

impl Iterator for Memchr<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index <= self.data.len() - self.pattern.0.len() {
            let mut offset = memchr::memchr(self.first, &self.data[self.index..])?;

            if offset >= self.prefix {
                offset -= self.prefix;
            }

            self.index += offset;

            if *self.pattern == self.data[self.index..] {
                let index = self.index;
                self.index += self.pattern.0.len();
                return Some(index);
            } else {
                self.index += 1;
            }
        }

        None
    }
}
