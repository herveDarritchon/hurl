use crate::core::core::Pos;

use super::error::Error;

pub type ParseResult<'a, T> = std::result::Result<T, Error>;
pub type ParseFunc<'a, T> = fn(&mut Parser) -> ParseResult<'a, T>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parser {
    pub buffer: Vec<char>,
    pub state: ParserState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParserState {
    pub cursor: usize,
    pub pos: Pos,
}

impl Parser {
    #[allow(dead_code)]
    pub fn init(s: &str) -> Parser {
        return Parser {
            buffer: s.chars().collect(),
            state: ParserState {
                cursor: 0,
                pos: Pos { line: 1, column: 1 },
            },
        };
    }
}

// region remaining
impl Parser {
    pub fn remaining(&self) -> String {
        return self.buffer.as_slice()[self.state.cursor..self.buffer.len()]
            .iter()
            .collect();
    }
}

#[test]
fn test_remaining() {
    let mut parser = Parser::init("caf\u{00e9}");
    parser.state = ParserState {
        cursor: 2,
        pos: Pos { line: 1, column: 3 },
    };
    assert_eq!(parser.remaining(), String::from("f\u{00e9}"));
    parser.state = ParserState {
        cursor: 2,
        pos: Pos { line: 1, column: 3 },
    };
}
// endregion

// region is_eof
impl Parser {
    pub fn is_eof(&self) -> bool {
        return self.state.cursor == self.buffer.len();
    }
}

#[test]
fn test_is_eof() {
    let mut parser = Parser::init("x");
    assert_eq!(parser.is_eof(), false);
    parser.next_char();
    assert_eq!(parser.is_eof(), true);
}
// endregion

// region next-char
impl Parser {
    pub fn next_char(&mut self) -> Option<char> {
        match self.buffer.get(self.state.cursor) {
            None => return None,
            Some(c) => {
                self.state.cursor += 1;
                if !is_combining_character(*c) {
                    self.state.pos.column += 1;
                }
                if *c == '\n' {
                    self.state.pos.column = 1;
                    self.state.pos.line += 1;
                }
                return Some(*c);
            }
        }
    }

//    pub fn rewind(&mut self) {
//        self.state.cursor -= 1;
//        match self.buffer.get(self.state.cursor) {
//            None => {},
//            Some(c) => {
//
//                if !is_combining_character(*c) {
//                    self.state.pos.column -= 1;
//                }
//                if *c == '\n' {
//                    self.state.pos.column = 1;
//                    self.state.pos.line -= 1;
//                }
//            }
//        }
//    }
}

#[test]
fn test_next_char() {
    let mut parser = Parser::init("caf\u{00e9}");
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 0,
            pos: Pos { line: 1, column: 1 }
        }
    );
    assert_eq!(parser.next_char().unwrap(), 'c');
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 1,
            pos: Pos { line: 1, column: 2 }
        }
    );
    assert_eq!(parser.next_char().unwrap(), 'a');
    assert_eq!(parser.next_char().unwrap(), 'f');
    assert_eq!(parser.next_char().unwrap(), '\u{00e9}');
    assert_eq!(parser.next_char(), None);
}
// endregion

// region next-chars
impl Parser {
    pub fn next_chars(&mut self, count: usize) -> Option<String> {
        let mut count = count;
        let mut s = String::from("");
        while count > 0 {
            match self.next_char() {
                None => return None,
                Some(c) => {
                    s.push(c);
                }
            }
            count = count - 1;
        }
        return Some(s);
    }
}

#[test]
fn test_next_chars() {
    let mut parser = Parser::init("caf\u{00e9}");
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 0,
            pos: Pos { line: 1, column: 1 }
        }
    );
    assert_eq!(parser.next_chars(4).unwrap(), "caf\u{00e9}");
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 4,
            pos: Pos { line: 1, column: 5 }
        }
    );

    let mut parser = Parser::init("caf\u{0065}\u{0301}");
    assert_eq!(parser.next_chars(5).unwrap(), "caf\u{0065}\u{0301}");
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 5,
            pos: Pos { line: 1, column: 5 }
        }
    );
}
// endregion

// TODO check its usage => return empty string makes senses sometimes??
// region next_graphemes_while
// can not fail is reflected into its type
// whitelist parsing
impl<'a> Parser {
    pub fn next_chars_while(&mut self, predicate: fn(&char) -> bool) -> String {
        let mut s = String::from("");

        loop {
            let save_state = self.state.clone();
            match self.next_char() {
                None => return s,
                Some(c) => {
                    if !predicate(&c) {
                        self.state = save_state;
                        return s;
                    }
                    s.push(c);
                }
            }
        }
    }
}

#[test]
fn test_next_chars_while() {
    let mut parser = Parser::init("param1:value1");
    assert_eq!(parser.next_chars_while(|c| c.is_alphanumeric()), "param1");
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 6,
            pos: Pos { line: 1, column: 7 }
        }
    );

    let mut parser = Parser::init("param1");
    assert_eq!(parser.next_chars_while(|c| c.is_alphanumeric()), "param1");
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 6,
            pos: Pos { line: 1, column: 7 }
        }
    );

    let mut parser = Parser::init("");
    assert_eq!(parser.next_chars_while(|c| c.is_alphanumeric()), "");
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 0,
            pos: Pos { line: 1, column: 1 }
        }
    );
}
// endregion

// region next_char_until
// can not fail is reflected into its type
impl<'a> Parser {
    pub fn next_chars_until(&mut self, predicate: fn(&char) -> bool) -> String {
        let mut s = String::from("");

        loop {
            let save_state = self.state.clone();
            match self.next_char() {
                None => return s,
                Some(c) => {
                    if predicate(&c) {
                        self.state = save_state;
                        return s;
                    }
                    s.push(c);
                }
            }
        }
    }
}

#[test]
fn test_next_char_until() {
    let mut parser = Parser::init("param1:value1");
    assert_eq!(parser.next_chars_until(|c| *c == ':'), "param1");
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 6,
            pos: Pos { line: 1, column: 7 }
        }
    );

    let mut parser = Parser::init("param1");
    assert_eq!(parser.next_chars_until(|c| *c == ':'), "param1");
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 6,
            pos: Pos { line: 1, column: 7 }
        }
    );

    let mut parser = Parser::init("");
    assert_eq!(parser.next_chars_until(|c| *c == ':'), "");
    assert_eq!(
        parser.state,
        ParserState {
            cursor: 0,
            pos: Pos { line: 1, column: 1 }
        }
    );
}
// endregion

// region combining unicode

fn is_combining_character(c: char) -> bool {
    return c > '\u{0300}' && c < '\u{036F}'; // Combining Diacritical Marks (0300–036F)
}

#[test]
fn test_e_acute() {
    // one unicode code point
    let bytes = vec![195, 169]; // utf8 encoding: c3 a9
    let s = std::str::from_utf8(&bytes).unwrap();
    assert_eq!(s, "\u{00e9}");
    assert_eq!(s.len(), 2);
    //assert_eq!(graphemes(s), vec!["\u{00e9}"]);

    // using combining unicode codepoint
    let bytes = vec![101, 204, 129]; // utf8 encoding: 65 cc 81
    let s = std::str::from_utf8(&bytes).unwrap();
    assert_eq!(s, "\u{0065}\u{0301}");
    assert_eq!(s.len(), 3);
    //assert_eq!(graphemes(s), vec!["\u{0065}\u{0301}"]);
}

// endregion
