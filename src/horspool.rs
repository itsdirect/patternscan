use crate::pattern::{MaskedByte, Pattern};

pub struct Horspool<'a> {
    pattern: &'a Pattern,
    data: &'a [u8],
    skip_table: Vec<usize>,
    index: usize,
}

impl<'a> Horspool<'a> {
    pub fn new(pattern: &'a Pattern, data: &'a [u8]) -> Self {
        Self {
            pattern,
            data,
            skip_table: build_skip_table(pattern),
            index: 0,
        }
    }
}

impl Iterator for Horspool<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index <= self.data.len() - self.pattern.0.len() {
            if *self.pattern == self.data[self.index..] {
                let index = self.index;
                self.index += self.pattern.0.len();
                return Some(index);
            }

            let last_index = self.pattern.0.len() - 1;
            self.index += self.skip_table[self.data[self.index + last_index] as usize];
        }

        None
    }
}

fn build_skip_table(pattern: &Pattern) -> Vec<usize> {
    let last_index = pattern.0.len() - 1;
    let mut max_skip = pattern.0.len();

    for i in 0..last_index {
        if let MaskedByte(None) = pattern.0[i] {
            max_skip = last_index - i;
        }
    }

    let mut skip_table = vec![max_skip; 256];

    for i in 0..last_index {
        if let MaskedByte(Some(byte)) = pattern.0[i] {
            skip_table[byte as usize] = last_index - i;
        }
    }

    skip_table
}
