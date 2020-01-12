use serde_json;

use crate::core::core::Pos;

use super::core::*;
use super::error;
use super::primitives::*;

#[cfg(test)]
use std::error::Error;

// region json-char
pub fn json_char(p: &mut Parser) -> ParseResult<'static, Option<(char, String)>> {
    let start = p.clone().state;
    return match p.next_char() {
        None => Ok(None),
        Some('"') => {
            p.state = start;
            Ok(None)
        }
        Some('\\') => {
            let (c, encoded) = escape_sequence(p)?;
            Ok(Some((c, format!("\\{}", encoded))))
        }
        Some(c) => {
            if c.is_ascii_control() {
                Ok(None)
            } else {
                Ok(Some((c, c.to_string())))
            }
        }
    };
}

#[test]
fn test_json_char() {
    let mut parser = Parser::init("a");
    assert_eq!(
        json_char(&mut parser).unwrap(),
        Some(('a', String::from("a")))
    );
    assert_eq!(parser.state.cursor, 1);

    let mut parser = Parser::init("\\\"");
    assert_eq!(
        json_char(&mut parser).unwrap(),
        Some(('"', String::from("\\\"")))
    );
    assert_eq!(parser.state.cursor, 2);

    let mut parser = Parser::init("\\n");
    assert_eq!(
        json_char(&mut parser).unwrap(),
        Some(('\n', String::from("\\n")))
    );
    assert_eq!(parser.state.cursor, 2);

    let mut parser = Parser::init("\\u00e9");
    assert_eq!(
        json_char(&mut parser).unwrap(),
        Some(('é', String::from("\\u00e9")))
    );
    assert_eq!(parser.state.cursor, 6);
}

#[test]
fn test_json_char_none() {
    let mut parser = Parser::init("\"");
    assert_eq!(json_char(&mut parser).unwrap(), None);
    assert_eq!(parser.state.cursor, 0);
}

#[test]
fn test_json_char_error() {
    let mut parser = Parser::init("\\u00xx");
    let error = json_char(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 5 });
    assert_eq!(
        error.inner,
        error::ParseError::Unexpected {
            character: String::from("x")
        }
    );
}

// endregion

// region escape-char
fn escape_sequence(p: &mut Parser) -> ParseResult<'static, (char, String)> {
    return match p.next_char() {
        Some('"') => Ok(('"', String::from("\""))),
        Some('\\') => Ok(('"', String::from("\\"))),
        Some('/') => Ok(('"', String::from("/"))),
        Some('b') => Ok(('\x08', String::from("b"))),
        Some('n') => Ok(('\n', String::from("n"))),
        Some('f') => Ok(('\x0c', String::from("f"))),
        Some('r') => Ok(('\r', String::from("r"))),
        Some('t') => Ok(('\t', String::from("t"))),
        Some('u') => match unicode(p) {
            Ok((c, s)) => Ok((c, format!("u{}", s))),
            Err(e) => Err(e),
        },
        _ => Err(error::Error {
            pos: p.clone().state.pos,
            recoverable: false,
            inner: error::ParseError::Json {},
        }),
    };
}

// endregion

// region unicode
fn unicode(p: &mut Parser) -> ParseResult<'static, (char, String)> {
    let d1 = hex_digit(p)?;
    let d2 = hex_digit(p)?;
    let d3 = hex_digit(p)?;
    let d4 = hex_digit(p)?;
    let v = hex_digit_value(d1) * 16 * 16 * 16
        + hex_digit_value(d2) * 16 * 16
        + hex_digit_value(d3) * 16
        + hex_digit_value(d4);
    return match std::char::from_u32(v) {
        None => Err(error::Error {
            pos: p.clone().state.pos,
            recoverable: false,
            inner: error::ParseError::Json {},
        }),
        Some(c) => Ok((c, format!("{}{}{}{}", d1, d2, d3, d4))),
    };
}

fn hex_digit_value(c: char) -> u32 {
    return match c.to_ascii_lowercase() {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => panic!("invalid hex digit"),
    };
}

#[test]
fn test_unicode() {
    let mut parser = Parser::init("000a");
    assert_eq!(unicode(&mut parser).unwrap(), ('\n', String::from("000a")));

    let mut parser = Parser::init("00E9");
    assert_eq!(unicode(&mut parser).unwrap(), ('é', String::from("00E9")));
}
// endregion

// region hex_digit
fn hex_digit(p: &mut Parser) -> ParseResult<'static, char> {
    let start = p.clone().state;
    return match p.next_char() {
        Some(c) => {
            if (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F') {
                Ok(c)
            } else {
                Err(error::Error {
                    pos: start.pos,
                    recoverable: false,
                    inner: error::ParseError::Unexpected {
                        character: c.to_string(),
                    },
                })
            }
        }
        None => Err(error::Error {
            pos: start.pos,
            recoverable: false,
            inner: error::ParseError::Json {},
        }),
    };
}

#[test]
fn test_hex_digit() {
    let mut parser = Parser::init("0");
    assert_eq!(hex_digit(&mut parser).unwrap(), '0');
}
// endregion

// region json-string

pub fn json_string2(p: &mut Parser) -> ParseResult<'static, (String, String)> {
    //  let start = p.state.clone();
    let mut value = String::from("");
    let mut encoded = String::from("");

    try_literal("\"", p)?;
    loop {
        match json_char(p) {
            Ok(None) => break,
            Ok(Some((c, s))) => {
                value.push(c);
                encoded.push_str(s.as_str());
            }
            Err(e) => return Err(e),
        }
    }
    literal("\"", p)?;
    return Ok((value, encoded));
}

#[test]
fn test_json_string2() {
    let mut parser = Parser::init("\"\"");
    assert_eq!(
        json_string2(&mut parser).unwrap(),
        (String::from(""), String::from(""))
    );
    assert_eq!(parser.state.cursor, 2);

    let mut parser = Parser::init("\"caf\\u00e9\"");
    assert_eq!(
        json_string2(&mut parser).unwrap(),
        (String::from("café"), String::from("caf\\u00e9"))
    );
    assert_eq!(parser.state.cursor, 11);
}

// does not start with a whitespace
// not recoverable if the first character is a double quote!
// return decoded string - client should extract source innfo and encoded text if needed
pub fn json_string(p: &mut Parser) -> ParseResult<'static, String> {
    let start = p.state.clone();
    let s = p.clone().remaining();
    let mut stream =
        serde_json::Deserializer::from_str(s.as_str()).into_iter::<serde_json::Value>();
    return match stream.next() {
        Some(Ok(serde_json::Value::String(v))) => {
            let unicode_points: Vec<char> =
                std::str::from_utf8(&s.as_bytes()[0..stream.byte_offset()])
                    .unwrap()
                    .chars()
                    .collect();
            p.next_chars(unicode_points.len());
            Ok(v)
        }
        Some(Ok(_)) => {
            //println!("case0");
            Err(error::Error {
                pos: start.pos,
                recoverable: true,
                inner: error::ParseError::Json {},
            })
        }
        Some(Err(e)) => {
            //println!("case1 {:?}", e);
            if e.is_eof() {
                Err(error::Error {
                    pos: Pos {
                        line: e.line(),
                        column: e.column() + 1,
                    },
                    recoverable: false,
                    inner: error::ParseError::Eof {},
                })
            } else {
                Err(error::Error {
                    pos: start.pos,
                    recoverable: true,
                    inner: error::ParseError::Json {},
                })
            }
        }
        None => {
            //println!("case2 {}", stream.byte_offset());
            Err(error::Error {
                pos: start.pos,
                recoverable: true,
                inner: error::ParseError::Eof {},
            })
        }
    };
}

#[test]
fn test_json_string() {
    let mut parser = Parser::init("\"Foo\"");
    assert_eq!(json_string2(&mut parser).unwrap().1, String::from("Foo"));
    assert_eq!(parser.state.cursor, 5);
}

#[test]
fn test_json_string_error_recoverable() {
    let mut parser = Parser::init("xxx");
    let error = json_string2(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, error::ParseError::Expecting {value: String::from("\"")});
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("");
    let error = json_string2(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, error::ParseError::Expecting {value: String::from("\"")});
    assert_eq!(error.recoverable, true);
}

#[test]
fn test_json_string_error_non_recoverable() {
    let mut parser = Parser::init("\"xxx");
    let error = json_string2(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 5 });
    assert_eq!(error.inner, error::ParseError::Expecting {value: String::from("\"")});
    assert_eq!(error.recoverable, false);
}
// endregion

// region json

// json value
// use first your primitive boolean, integer, float
// for better error handling
pub fn json_value(p: &mut Parser) -> ParseResult<'static, String> {

    let start = p.state.clone();

    match boolean(p) {
        Ok(r) => return Ok(r.to_string()),
        _ => { p.state = start.clone();},
    }

    let s = p.clone().remaining().clone();

    if p.clone().remaining().as_str().starts_with(" ")
        || p.clone().remaining().as_str().starts_with("\t")
    {
        panic!("json value should not start with whitespace");
    }

    if p.is_eof() || (!p.remaining().starts_with("{") && !p.remaining().starts_with("[") && !p.remaining().starts_with("\"")) {
        return Err(error::Error {
            pos: start.pos,
            recoverable: true,
            inner: error::ParseError::Json {},
        });
    }

    let mut stream =
        serde_json::Deserializer::from_str(s.as_str()).into_iter::<serde_json::Value>();
    return match stream.next() {
        None => Err(error::Error {
            pos: start.pos,
            recoverable: false,
            inner: error::ParseError::Json {},
        }),
        Some(r) => match r {
            Ok(_) => {
                let unicode_points: Vec<char> =
                    std::str::from_utf8(&s.as_bytes()[0..stream.byte_offset()])
                        .unwrap()
                        .chars()
                        .collect();
                let encoded = p.next_chars(unicode_points.len()).unwrap();
                Ok(encoded)
            }
            Err(e) => {

                // hacky
                // find position one character before error
                p.state = start.clone();
                let error_line = start.pos.line + e.line() - 1;
                let error_column = start.pos.column + e.column();
                let mut count = 0;
                while p.state.pos.line != error_line || p.state.pos.column != error_column{
                    p.next_char();
                    count += 1;
                    //eprintln!(">> {:?}", p.state.pos);
                }
                //eprintln!("s.len() {}", s.len());
                //eprintln!("count {}", count);
                let pos = if s.len()== count { // end of file
                    Pos { line: error_line, column: error_column}
                } else {
                    p.state = start.clone();
                    for _ in 0..(count-2) {
                        p.next_char();
                    }
                    p.state.clone().pos
                };

                Err(error::Error {
                    pos,
                    recoverable: false,
                    inner: error::ParseError::Json {},
                })
            },
        },
    };
}

// used by the body
// may start with whitespace - valid json?
//
//pub fn json(p: &mut Parser) -> ParseResult<'static, String> {
//    let start = p.state.clone();
//
//    // though json can start with whitespace
//    // you prefer to keep separate
//    // prefer to fail in case of whitespace
//    if p.is_eof() {
//        return Err(error::Error {
//            pos: p.state.clone().pos,
//            recoverable: true,
//            inner: error::ParseError::Json {},
//        });
//    }
//    if p.clone().remaining().as_str().starts_with(" ")
//        || p.clone().remaining().as_str().starts_with("\t")
//    {
//        return Err(error::Error {
//            pos: p.state.clone().pos,
//            recoverable: true,
//            inner: error::ParseError::Unexpected {
//                character: String::from("whitespace"),
//            },
//        });
//    }
//
//    // use serde_json only for string/list and object
//    if p.clone().remaining().as_str().starts_with("\"")
//        || p.clone().remaining().as_str().starts_with("{")
//        || p.clone().remaining().as_str().starts_with("[")
//    {
//        let s = p.clone().remaining().clone();
//        let mut stream =
//            serde_json::Deserializer::from_str(s.as_str()).into_iter::<serde_json::Value>();
//        return match stream.next() {
//            None => Err(error::Error {
//                pos: start.pos,
//                recoverable: false,
//                inner: error::ParseError::Json {},
//            }),
//            Some(r) => match r {
//                Ok(_) => {
//                    let unicode_points: Vec<char> =
//                        std::str::from_utf8(&s.as_bytes()[0..stream.byte_offset()])
//                            .unwrap()
//                            .chars()
//                            .collect();
//                    let encoded = p.next_chars(unicode_points.len()).unwrap();
//                    Ok(encoded)
//                }
//                Err(e) => Err(error::Error {
//                    pos: Pos {
//                        line: start.pos.line + e.line() - 1,
//                        column: start.pos.column + e.column() - 1,
//                    },
//                    recoverable: (e.line() == 1) && (e.column() == 1),
//                    inner: error::ParseError::Json {},
//                }),
//            },
//        };
//    } else {
//        let available_literals = vec!["true", "false"];
//        for s in available_literals {
//            match try_literal(s, p) {
//                Ok(_) => return Ok(s.to_string()),
//                _ => {}
//            }
//        }
//        return Err(error::Error {
//            pos: p.state.clone().pos,
//            recoverable: true,
//            inner: error::ParseError::Json {},
//        });
//    }
//}

#[test]
fn test_json() {
    let mut parser = Parser::init("\"Foo\"");
    assert_eq!(json_value(&mut parser).unwrap(), "\"Foo\"");
    assert_eq!(parser.state.cursor, 5);

    let mut parser = Parser::init("\"Foo\" ");
    assert_eq!(json_value(&mut parser).unwrap(), "\"Foo\"");
    assert_eq!(parser.state.cursor, 5);

    let mut parser = Parser::init("{}xx");
    assert_eq!(json_value(&mut parser).unwrap(), "{}");
    assert_eq!(parser.state.cursor, 2);
}

#[test]
fn test_json_bool() {
    let mut parser = Parser::init("true x");
    assert_eq!(json_value(&mut parser).unwrap(), "true");
    assert_eq!(parser.state.cursor, 4);
}

#[test]
fn test_json_error_recoverable() {
    let mut parser = Parser::init("x");
    let error = json_value(&mut parser).err().unwrap();
    assert_eq!(error.pos.line, 1);
    assert_eq!(error.pos.column, 1);
    assert_eq!(error.recoverable, true);
    assert_eq!(parser.state.cursor, 0);
}

#[test]
fn test_json_error() {
    let mut parser = Parser::init("{x");
    let error = json_value(&mut parser).err().unwrap();
    assert_eq!(error.pos.line, 1);
    assert_eq!(error.pos.column, 3);
    assert_eq!(error.recoverable, false);
}


#[test]
fn test_json_error_eof() {
    let mut parser = Parser::init("{ \"name\":");
    let error = json_value(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos {line: 1, column: 10});
    assert_eq!(error.recoverable, false);
}
#[test]
fn test_json_error_eof2() {
    let mut parser = Parser::init("{ \"name\":\n");
    let error = json_value(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos {line: 2, column: 1});
    assert_eq!(error.recoverable, false);
}

#[test]
fn test_json_error_eol() {
    let mut parser = Parser::init("{ \"name\":\nHTTP/1.1 200");
    let error = json_value(&mut parser).err().unwrap();
    println!("{:?}", error);
    assert_eq!(error.pos, Pos { line: 1, column: 10});
    assert_eq!(error.recoverable, false);
}

#[test]
fn test_json_error_unrecoverable() {
    let mut parser = Parser::init("{");
    let error = json_value(&mut parser).err().unwrap();
    assert_eq!(error.pos.line, 1);
    assert_eq!(error.pos.column, 2);
    assert_eq!(error.recoverable, false);
}

/*
   recoverabl json error
   example file
   => but f can be used for false
   may have to customize it?

*/
//#[test]
//fn test_json_error2() {
//    let mut parser = Parser::init("file;data.xml;");
//    let error = json(&mut parser).err().unwrap();
//    assert_eq!(error.pos.line, 1);
//    assert_eq!(error.pos.column, 1);
//    assert_eq!(error.recoverable, true);
//    assert_eq!(parser.state.cursor, 0);
//}
// endregion

// region serde_json

#[test]
fn test_serde_json() {
    let mut stream = serde_json::Deserializer::from_str("\"Foo\"").into_iter::<serde_json::Value>();
    assert_eq!(
        serde_json::Value::String(String::from("Foo")),
        stream.next().unwrap().unwrap()
    );
    assert_eq!(stream.byte_offset(), 5);

    let mut stream =
        serde_json::Deserializer::from_str("\"Foo\" ").into_iter::<serde_json::Value>();
    assert_eq!(
        serde_json::Value::String("Foo".to_string()),
        stream.next().unwrap().unwrap()
    );
    assert_eq!(stream.byte_offset(), 5);

    let mut stream =
        serde_json::Deserializer::from_str(" \"Foo\" ").into_iter::<serde_json::Value>();
    assert_eq!(
        serde_json::Value::String("Foo".to_string()),
        stream.next().unwrap().unwrap()
    );
    assert_eq!(stream.byte_offset(), 6);

    let mut stream =
        serde_json::Deserializer::from_str("\n \"Foo\" ").into_iter::<serde_json::Value>();
    assert_eq!(
        serde_json::Value::String("Foo".to_string()),
        stream.next().unwrap().unwrap()
    );
    assert_eq!(stream.byte_offset(), 7);
}

#[test]
fn test_serde_json_error() {
    let mut stream = serde_json::Deserializer::from_str("\"Foo").into_iter::<serde_json::Value>();
    let error = stream.next().unwrap().err().unwrap();
    assert_eq!(error.description(), "JSON error");
    assert_eq!(error.column(), 4);
    assert!(error.is_eof());
    assert_eq!(stream.byte_offset(), 0);

    let mut stream = serde_json::Deserializer::from_str("{ x").into_iter::<serde_json::Value>();
    let error = stream.next().unwrap().err().unwrap();
    //println!("{:?}", error);
    assert_eq!(error.description(), "JSON error");
    assert_eq!(error.column(), 3);
    assert!(error.is_syntax());
    assert_eq!(stream.byte_offset(), 0);
}

pub fn json_chars(s: String) -> Vec<String> {
    //println!(">> {:?}", s);
    let mut chars = vec![];
    let mut characters = s.chars();
    loop {
        match characters.next() {
            None => {
                break;
            }
            Some(c) => {
                match c {
                    '\\' => {
                        match characters.next() {
                            None => {
                                break;
                            } // should not happen
                            Some('u') => {
                                let c1 = characters.next().unwrap();
                                let c2 = characters.next().unwrap();
                                let c3 = characters.next().unwrap();
                                let c4 = characters.next().unwrap();
                                chars.push(format!("\\u{}{}{}{}", c1, c2, c3, c4));
                            }
                            Some(c) => {
                                chars.push(format!("\\{}", c));
                            }
                        }
                    }
                    _ => chars.push(c.to_string()),
                }
            }
        }
    }
    return chars;
}

pub fn json_characters(s: String) -> Vec<(char, String)> {
    let mut chars = vec![];
    let mut characters = s.chars();
    loop {
        match characters.next() {
            None => {
                break;
            }
            Some(c) => {
                match c {
                    '\\' => {
                        match characters.next() {
                            None => {
                                break;
                            } // should not happen
                            Some('u') => {
                                let c1 = characters.next().unwrap();
                                let c2 = characters.next().unwrap();
                                let c3 = characters.next().unwrap();
                                let c4 = characters.next().unwrap();
                                chars.push((c, format!("\\u{}{}{}{}", c1, c2, c3, c4)));
                            }
                            Some(c) => {
                                chars.push((c, format!("\\{}", c)));
                            }
                        }
                    }
                    _ => chars.push((c, c.to_string())),
                }
            }
        }
    }
    return chars;
}

#[test]
fn test_serde_json_character() {
    let s = "\"A\\u0041\\n\\u000a\"xxx";
    assert_eq!(s.chars().count(), 20);
    let mut stream = serde_json::Deserializer::from_str(s).into_iter::<serde_json::Value>();
    let value = stream.next().unwrap().unwrap();
    assert!(value.is_string());
    assert_eq!(value.as_str().unwrap().chars().count(), 4);
    assert_eq!(stream.byte_offset(), 17);
    assert_eq!(
        json_chars(s[1..stream.byte_offset() - 1].to_string()),
        vec!["A", "\\u0041", "\\n", "\\u000a"]
    );
}
// endregion
