use crate::field::Field;

const NULL_CHAR: char = 0 as char;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Header {
    pub field: Field,
    pub position: Position,
}

#[derive(Clone, Copy, Debug, PartialEq)]
// Represents a position in a string
// start_col is inclusive, end_col is exclusive i.e (0..4) means 0, 1, 2, 3
// Positions are zero-indexed and meant to be used with the string slice like this:
// string[Position.start_col..Position.end_col]
pub(crate) struct Position {
    pub start_col: usize,
    pub end_col: usize,
}

pub(crate) struct Parser {
    input: String,
    current_char: char,
    current_pos: usize,
    read_pos: usize,
}

impl Header {
    pub fn new(field: Field, position: Position) -> Self {
        return Self { field, position };
    }
}

impl Position {
    pub fn new(start_col: usize, end_col: usize) -> Self {
        return Self { start_col, end_col };
    }
}

// it is more of a lexer but whatever, it does do a bit (or actually a lot) of parsing
impl Parser {
    pub fn new(input: String) -> Self {
        let mut parser = Self {
            input,
            current_char: NULL_CHAR,
            current_pos: 0,
            read_pos: 0,
        };

        parser.read_char();
        return parser;
    }

    fn read_char(&mut self) {
        // if the read_pos is past the input length, set the current_char to 0 (as a null char)
        if self.read_pos >= self.input.len() {
            self.current_char = NULL_CHAR;
        } else {
            self.current_char = self.input.chars().nth(self.read_pos).unwrap_or(NULL_CHAR);
        }

        self.current_pos = self.read_pos;
        self.read_pos += 1;
    }

    fn read_ident(&mut self) -> String {
        let start_pos = self.current_pos;
        while self.current_char.is_alphanumeric() {
            self.read_char();
        }

        return self.input[start_pos..self.current_pos].to_string();
    }

    // Parse the header line to figure out where the fields are (by column)
    pub fn parse(&mut self) -> Vec<Header> {
        let mut headers: Vec<Header> = Vec::new();

        loop {
            if self.current_char == NULL_CHAR {
                break;
            }

            // We do not want to eat whitespace, we care about it as we consider it part of our
            // position(s)
            if self.current_char.is_whitespace() {
                self.read_char();
                continue;
            }

            let ident = self.read_ident();
            // Our start position begins right after where the last column ended - including
            // whitespace, if we have only just begin parsing, then it is going to be zero
            let start_col = match headers.last() {
                Some(last_header) => last_header.position.end_col,
                None => 0,
            };
            let position = Position::new(start_col, self.current_pos);
            headers.push(Header::new(ident.to_lowercase().into(), position));
            continue;
        }

        return headers;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Field::*;

    macro_rules! header {
        ($field:expr, $start_col:expr, $end_col:expr) => {
            Header::new($field, Position::new($start_col, $end_col))
        };
    }

    macro_rules! slice {
        ($string:expr, $header:expr, $index:expr) => {
            $string[$header[$index].position.start_col..$header[$index].position.end_col]
                .to_string()
        };
    }

    #[test]
    fn test_read_ident() {
        let mut parser = Parser::new("hello world".into());
        assert_eq!(parser.read_ident(), "hello");
    }

    #[test]
    fn test_parse() {
        let test_header_string = " PID  PPID      TIME COMM";
        let mut parser = Parser::new(test_header_string.into());
        let got = parser.parse();

        let expected: Vec<Header> = vec![
            header!(Pid, 0, 4),
            header!(Ppid, 4, 10),
            header!(Time, 10, 20),
            header!(Comm(crate::field::Command::FullPath), 20, 25),
        ];

        assert_eq!(got, expected);
        assert_eq!(slice!(test_header_string, got, 0), " PID");
        assert_eq!(slice!(test_header_string, got, 1), "  PPID");
        assert_eq!(slice!(test_header_string, got, 2), "      TIME");
        assert_eq!(slice!(test_header_string, got, 3), " COMM");
    }

    #[test]
    fn test_parse_any_position() {
        let test_header_string = "COMM PID   PPID";
        let mut parser = Parser::new(test_header_string.into());
        let got = parser.parse();
        let expected: Vec<Header> = vec![
            header!(Comm(crate::field::Command::FullPath), 0, 4),
            header!(Pid, 4, 8),
            header!(Ppid, 8, 15),
        ];

        assert_eq!(got, expected);
        assert_eq!(slice!(test_header_string, got, 0), "COMM");
        assert_eq!(slice!(test_header_string, got, 1), " PID");
        assert_eq!(slice!(test_header_string, got, 2), "   PPID");
    }
}
