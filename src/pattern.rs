use std::num::ParseIntError;
use std::str::FromStr;

use crate::{Memchr, Searcher};

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
    pub fn matches(&'a self, data: &'a [u8]) -> impl Searcher {
        self.matches_with_searcher::<Memchr>(data)
    }

    pub fn matches_with_searcher<S: Searcher<'a>>(&'a self, data: &'a [u8]) -> S {
        S::new(self, data)
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
