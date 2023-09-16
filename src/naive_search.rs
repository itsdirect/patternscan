use crate::pattern::Pattern;

pub struct NaiveSearch<'a> {
    pattern: &'a Pattern,
    data: &'a [u8],
    index: usize,
}

impl<'a> NaiveSearch<'a> {
    pub fn new(pattern: &'a Pattern, data: &'a [u8]) -> Self {
        Self {
            pattern,
            data,
            index: 0,
        }
    }
}

impl Iterator for NaiveSearch<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index <= self.data.len() - self.pattern.0.len() {
            if *self.pattern == self.data[self.index..] {
                let index = self.index;
                self.index += self.pattern.0.len();
                return Some(index);
            }

            self.index += 1;
        }

        None
    }
}
