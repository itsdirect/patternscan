use std::num::ParseIntError;
use std::str::FromStr;

pub struct MaskedByte(pub Option<u8>);

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

pub struct Pattern(pub Vec<MaskedByte>);

impl<'a> Pattern {
    pub fn matches(&'a self, data: &'a [u8]) -> impl Iterator<Item = usize> + use<'a> {
        Matches::new(self, data)
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
    prefix: usize,
    first: u8,
    data: &'a [u8],
    index: usize,
}

impl<'a> Matches<'a> {
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

impl Iterator for Matches<'_> {
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
