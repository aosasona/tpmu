use crate::field::Field;

// steps to parse
// - we need to figure out where the single & multi-word fields are
// - we need to use that to figure out where the fields are and hence where the values are

pub(crate) struct Tokenizer {
    input: Vec<u8>,
    current_char: u8,
    current_pos: usize,
    read_pos: usize,
    pub expected_fields: Vec<Field>,
}

impl Tokenizer {
    pub fn new(input: Vec<u8>, fields: Vec<Field>) -> Self {
        let t = Self {
            input,
            current_char: 0,
            current_pos: 0,
            read_pos: 0,
            expected_fields: fields,
        };

        t.read_char();
        t
    }

    pub fn tokenize(&self) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    fn read_char(&self) {
        if self.read_pos >= self.input.len() {}
    }
}

mod tests {}
