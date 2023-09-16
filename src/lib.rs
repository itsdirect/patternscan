use std::num::ParseIntError;
use std::str::FromStr;

pub struct MaskedByte(Option<u8>);

impl FromStr for MaskedByte {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "?" {
            return Ok(Self(None));
        }

        let byte = u8::from_str_radix(s, 16)?;
        Ok(Self(Some(byte)))
    }
}

impl PartialEq<u8> for MaskedByte {
    fn eq(&self, other: &u8) -> bool {
        match self.0 {
            Some(byte) => byte == *other,
            None => true,
        }
    }
}

pub struct Pattern(Vec<MaskedByte>);

impl<'a> Pattern {
    pub fn matches(&'a self, data: &'a [u8]) -> Matches {
        Matches {
            pattern: self,
            data,
            skip_table: build_skip_table(self),
            index: 0,
        }
    }
}

impl FromStr for Pattern {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(bytes))
    }
}

impl PartialEq<[u8]> for Pattern {
    fn eq(&self, other: &[u8]) -> bool {
        if self.0.len() > other.len() {
            return false;
        }

        self.0.iter().zip(other.iter()).rev().all(|(a, b)| a == b)
    }
}

pub struct Matches<'a> {
    pattern: &'a Pattern,
    data: &'a [u8],
    skip_table: Vec<usize>,
    index: usize,
}

impl Iterator for Matches<'_> {
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
