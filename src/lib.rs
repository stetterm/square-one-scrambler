//! Main component for creating and
//! interacting with Square-One
//! cubes and scrambling them.

pub mod cube;

use std::fmt;

pub struct Scramble {
    turns: Vec<(i8, i8)>,
}

impl fmt::Display for Scramble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for i in 0..self.turns.len()-1 {
            output = [output, format!("{:?} / ", self.turns[i])].concat();
        }
        output = [output, format!("{:?}", self.turns[self.turns.len()-1])].concat();
        write!(f, "{}", output.to_string())
    }
}