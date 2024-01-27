use std::collections::HashMap;

use crate::{
    field::Field,
    header::{Header, Position},
};

// steps to parse:
// - we need to figure out where the single & multi-word fields are
// - we need to use that to figure out where the fields are and hence where the values are
//

pub(crate) struct Tokenizer {
    input: String,
    current_char: u8,
    current_pos: usize,
    read_pos: usize,
    positions: HashMap<String, Vec<Position>>,
    pub headers: Vec<Header>,
}

impl Tokenizer {
    pub fn new(input: String, headers: Vec<Header>) -> Self {
        return Self {
            input,
            current_char: 0,
            current_pos: 0,
            read_pos: 0,
            headers,
            positions: HashMap::new(),
        };
    }

    pub fn tokenize(&self) -> Result<Vec<String>, String> {
        unimplemented!()
    }

    fn read_char(&self) {
        unimplemented!()
    }
}
