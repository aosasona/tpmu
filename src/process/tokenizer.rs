use std::collections::HashMap;

use crate::field::Field;

// steps to parse:
// - we need to figure out where the single & multi-word fields are
// - we need to use that to figure out where the fields are and hence where the values are

pub(crate) struct Tokenizer {
    input: String,
    current_char: u8,
    current_pos: usize,
    read_pos: usize,
    locations: HashMap<String, Vec<usize>>,
    pub expected_fields: Vec<Field>,
}

impl Tokenizer {
    pub fn new(input: String, fields: Vec<Field>) -> Self {
        let t = Self {
            input,
            current_char: 0,
            current_pos: 0,
            read_pos: 0,
            expected_fields: fields,
            locations: HashMap::new(),
        };

        t
    }

    pub fn tokenize(&self) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    fn read_char(&self) {
        unimplemented!()
    }

    fn calculate_expected_locations(&self) {
        let words = self.input.split_whitespace();
        // check the headers and use that to determine expected locations
    }
}

mod tests {}
