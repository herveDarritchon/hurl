use crate::core::ast::*;
use crate::core::core::SourceInfo;

use super::base64;
use super::combinators::*;
use super::core::*;
use super::error::*;
use super::expr;
use super::json;
use super::xml;

#[cfg(test)]
use crate::core::core::Pos;

// region space
pub fn space(p: &mut Parser) -> ParseResult<'static, Whitespace> {
    let start = p.state.clone();
    match p.next_char() {
        None => {
            return Err(Error {
                pos: start.pos,
                recoverable: true,
                inner: ParseError::Space {},
            });
        }
        Some(c) => {
            if c == ' ' || c == '\t' {
                return Ok(Whitespace {
                    value: c.to_string(),
                    source_info: SourceInfo::init(
                        start.pos.line,
                        start.pos.column,
                        p.state.pos.line,
                        p.state.pos.column,
                    ),
                });
            } else {
                return Err(Error {
                    pos: start.pos,
                    recoverable: true,
                    inner: ParseError::Space {},
                });
            }
        }
    }
}

#[test]
fn test_space() {
    let mut parser = Parser::init("x");
    let error = space(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(parser.state.cursor, 1);

    let mut parser = Parser::init("  ");
    assert_eq!(
        space(&mut parser),
        Ok(Whitespace {
            value: " ".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 2),
        }),
    );
    assert_eq!(parser.state.cursor, 1);
}

pub fn one_or_more_spaces<'a>(p: &mut Parser) -> ParseResult<'a, Whitespace> {
    let start = p.state.clone();
    match one_or_more(space, p) {
        Ok(v) => {
            let s = v.iter().map(|x| x.value.clone()).collect();
            return Ok(Whitespace {
                value: s,
                source_info: SourceInfo::init(
                    start.pos.line,
                    start.pos.column,
                    p.state.pos.line,
                    p.state.pos.column,
                ),
            });
        }
        Err(e) => return Err(e),
    }
}

#[test]
fn test_one_or_more_spaces() {
    let mut parser = Parser::init("  ");
    assert_eq!(
        one_or_more_spaces(&mut parser),
        Ok(Whitespace {
            value: "  ".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 3),
        })
    );

    let mut parser = Parser::init("abc");
    let error = one_or_more_spaces(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
}

pub fn zero_or_more_spaces<'a>(p: &mut Parser) -> ParseResult<'a, Whitespace> {
    let start = p.state.clone();
    match zero_or_more(space, p) {
        //Ok(v) => return Ok(v.join("")),
        Ok(v) => {
            let s = v.iter().map(|x| x.value.clone()).collect();
            return Ok(Whitespace {
                value: s,
                source_info: SourceInfo::init(
                    start.pos.line,
                    start.pos.column,
                    p.state.pos.line,
                    p.state.pos.column,
                ),
            });
        }
        Err(e) => return Err(e),
    };
}

#[test]
fn test_zero_or_more_spaces() {
    let mut parser = Parser::init("  ");
    assert_eq!(
        zero_or_more_spaces(&mut parser),
        Ok(Whitespace {
            value: "  ".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 3),
        })
    );
    assert_eq!(parser.state.cursor, 2);

    let mut parser = Parser::init("xxx");
    assert_eq!(
        zero_or_more_spaces(&mut parser),
        Ok(Whitespace {
            value: "".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 1),
        })
    );
    assert_eq!(parser.state.cursor, 0);

    let mut parser = Parser::init(" xxx");
    assert_eq!(
        zero_or_more_spaces(&mut parser),
        Ok(Whitespace {
            value: " ".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 2),
        })
    );
    assert_eq!(parser.state.cursor, 1);
}

// endregion

// region line terminator

pub fn line_terminator(p: &mut Parser) -> ParseResult<'static, LineTerminator> {
    // let start = p.state.clone();
    let space0 = zero_or_more_spaces(p)?;
    let comment = optional(|p1| comment(p1), p)?;
    let nl = if p.is_eof() {
        Whitespace {
            value: "".to_string(),
            source_info: SourceInfo::init(
                p.state.pos.line,
                p.state.pos.column,
                p.state.pos.line,
                p.state.pos.column,
            ),
        }
    } else {
        match newline(p) {
            Ok(r) => r,
            Err(e) => {
                return Err(Error {
                    pos: e.pos,
                    recoverable: false,
                    inner: ParseError::Expecting { value: String::from("line_terminator")},
                });
            }
        }
    };

    return Ok(LineTerminator {
        space0,
        comment,
        newline: nl,
    });
}

pub fn optional_line_terminators(p: &mut Parser) -> ParseResult<'static, Vec<LineTerminator>> {
    return zero_or_more(|p2| recover(|p1| line_terminator(p1), p2), p);
}

// endregion

// region comment

pub fn comment(p: &mut Parser) -> ParseResult<'static, Comment> {
    try_literal("#", p)?;
    let mut value = "".to_string();
    loop {
        if p.is_eof() {
            break;
        }
        let save_state = p.state.clone();
        match newline(p) {
            Ok(_) => {
                p.state = save_state;
                break;
            }
            _ => {
                p.state = save_state;
                match p.next_char() {
                    Some(c) => value.push(c),
                    _ => {}
                }
            }
        }
    }
    return Ok(Comment { value });
}

#[test]
fn test_comment() {
    //    let mut parser = Parser::init("# comment");
    //    assert_eq!(
    //        comment(&mut parser),
    //        Ok(Comment {
    //            value: " comment".to_string()
    //        })
    //    );
    //    assert_eq!(parser.state.cursor, 9);

    let mut parser = Parser::init("#\n");
    assert_eq!(
        comment(&mut parser),
        Ok(Comment {
            value: "".to_string()
        })
    );

    let mut parser = Parser::init("# comment\n");
    assert_eq!(
        comment(&mut parser),
        Ok(Comment {
            value: " comment".to_string()
        })
    );
    assert_eq!(parser.state.cursor, 9);

    let mut parser = Parser::init("xxx");
    let error = comment(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.recoverable, true);
}

// endregion

// region end_of_line

// may not consume characters
// consume trailing whitespace
//fn end_of_line(p: &mut Parser) -> ParseResult<'static, Whitespace> {
//    let start = p.state.clone();
//    match parsec::end_of_line(p) {
//        Some(s) => {
//            return Ok(Whitespace {
//                value: s + whitespace(p).unwrap().value.as_str(),
//                source_info: SourceInfo::init(
//                    start.pos.line,
//                    start.pos.column,
//                    p.state.pos.line,
//                    p.state.pos.column,
//                ),
//            });
//        }
//        None => Err(ParseError {
//            pos: start.pos,
//            expecting: "end of line".to_string(),
//            unexpected: "".to_string(),
//            recoverable: false,
//            current_line: "".to_string(),
//        }),
//    }
//}
//
//#[test]
//fn test_end_of_line() {
//    let mut parser = Parser::init("");
//    let eol = end_of_line(&mut parser).unwrap();
//    assert_eq!(eol.value, "".to_string());
//    assert_eq!(parser.state.cursor, 0);
//    assert_eq!(parser.state.pos.line, 1);
//    assert_eq!(parser.state.pos.column, 1);
//
//    let mut parser = Parser::init("\n\n   ");
//    let eol = end_of_line(&mut parser).unwrap();
//    assert_eq!(eol.value, "\n\n   ".to_string());
//    assert_eq!(parser.state.cursor, 5);
//    assert_eq!(parser.state.pos.line, 3);
//    assert_eq!(parser.state.pos.column, 4);
//
//    //
//    //    let mut parser = Parser::init("\r\n");
//    //    assert_eq!(end_of_line(&mut parser), Some("\r\n".to_string()));
//    //    assert_eq!(parser.state.cursor, 2);
//    //    assert_eq!(parser.state.pos.line, 2);
//    //    assert_eq!(parser.state.pos.column, 1);
//    //
//    //    let mut parser = Parser::init("\r");
//    //    assert_eq!(end_of_line(&mut parser), None);
//    //    assert_eq!(parser.state.cursor, 0);
//}
//
//#[test]
//fn test_end_of_line2() {
//    let mut parser =
//        Parser::init("GET http://google.fr # comment1\nGET http://google.fr # comment2");
//    parser.state.cursor = 31;
//    let s = end_of_line(&mut parser).unwrap();
//    assert_eq!(s.value, "\n");
//}

// endregion

// region hurl-template

// template with json
// HurlTemplate { with encoded text}
//

//pub fn hurl_template(p: &mut Parser) -> ParseResult<'static, HurlTemplate> {
//
//
//
//    return Ok(HurlTemplate{
//        elements: vec![],
//        encoded: "".to_string(),
//        source_info: SourceInfo::init(0,0,0,0)
//    });
//}

// endregion

// region literal
// does not return a value
// non recoverable parser
// => use combinator recover to make it recoverable
pub fn literal(s: &str, p: &mut Parser) -> ParseResult<'static, ()> {
    let start = p.state.clone();
    if p.clone().is_eof() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting { value: s.to_string() },
        });
    }
    for c in s.chars() {
        let _state = p.state.clone();
        match p.next_char() {
            None => {
                return Err(Error {
                    pos: start.pos,
                    recoverable: false,
                    inner: ParseError::Expecting { value: s.to_string() },
                });
            }
            Some(x) => {
                if x != c {
                    return Err(Error {
                        pos: start.pos,
                        recoverable: false,
                        inner: ParseError::Expecting { value: s.to_string() },
                    });
                } else {
                    continue;
                }
            }
        }
    }
    return Ok(());
}

#[test]
fn test_literal() {
    let mut parser = Parser::init("hello");
    assert_eq!(literal("hello", &mut parser), Ok(()));
    assert_eq!(parser.state.cursor, 5);

    let mut parser = Parser::init("");
    let error = literal("hello", &mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("hello") });
    assert_eq!(parser.state.cursor, 0);

    let mut parser = Parser::init("hi");
    let error = literal("hello", &mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("hello") });
    assert_eq!(parser.state.cursor, 2);

    let mut parser = Parser::init("he");
    let error = literal("hello", &mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("hello") });
    assert_eq!(parser.state.cursor, 2);
}

//
// Sugar for literal

// recoverable version which reset the cursor
// meant to be combined with following action
pub fn try_literal(s: &str, p: &mut Parser) -> ParseResult<'static, ()> {
    let save_state = p.state.clone();
    return match literal(s, p) {
        Ok(_) => Ok(()),
        Err(e) => {
            p.state = save_state;
            return Err(Error {
                pos: e.pos,
                recoverable: true,
                inner: e.inner,
            });
        }
    };
}

// peek version
//pub fn has_literal(s: &str, p: Parser) -> bool {
//    return peek(|p1| literal(s, p1), p).is_ok();
//}

// endregion

// region newline

pub fn newline(p: &mut Parser) -> ParseResult<'static, Whitespace> {
    let start = p.state.clone();
    return match try_literal("\r\n", p) {
        Ok(_) => Ok(Whitespace {
            value: "\r\n".to_string(),
            source_info: SourceInfo::init(
                start.pos.line,
                start.pos.column,
                p.state.pos.line,
                p.state.pos.column,
            ),
        }),
        Err(_) => match literal("\n", p) {
            Ok(_) => Ok(Whitespace {
                value: "\n".to_string(),
                source_info: SourceInfo::init(
                    start.pos.line,
                    start.pos.column,
                    p.state.pos.line,
                    p.state.pos.column,
                ),
            }),
            Err(_) => Err(Error {
                pos: start.pos,
                recoverable: false,
                inner: ParseError::Expecting {value: String::from("newline")},
            }),
        },
    };
}

#[test]
fn test_new_line() {
    let mut parser = Parser::init("\n");
    assert_eq!(
        newline(&mut parser).unwrap(),
        Whitespace {
            value: String::from("\n"),
            source_info: SourceInfo::init(1, 1, 2, 1),
        }
    );
}

// endregion

// region until-line-terminator

// can not fail => clear signature does not return an error
pub fn until_line_terminator(p: &mut Parser) -> String {
    let mut s = String::from("");
    loop {
        let save_state = p.state.clone();
        match line_terminator(p) {
            Ok(_) => {
                p.state = save_state;
                break;
            }
            _ => {
                p.state = save_state;
                match p.next_char() {
                    Some(c) => s.push(c),
                    _ => {}
                }
            }
        }
    }
    return s;
}

#[test]
fn test_until_line_terminator() {
    let mut parser = Parser::init("");
    assert_eq!(until_line_terminator(&mut parser), String::from(""));
    assert_eq!(parser.state.cursor, 0);

    let mut parser = Parser::init("x");
    assert_eq!(until_line_terminator(&mut parser), String::from("x"));
    assert_eq!(parser.state.cursor, 1);

    let mut parser = Parser::init("x\n");
    assert_eq!(until_line_terminator(&mut parser), String::from("x"));
    assert_eq!(parser.state.cursor, 1);

    let mut parser = Parser::init("x # comment");
    assert_eq!(until_line_terminator(&mut parser), String::from("x"));
    assert_eq!(parser.state.cursor, 1);
}

// endregion

// region name

pub fn name(p: &mut Parser) -> ParseResult<'static, HurlString> {
    let start = p.state.clone();
    return match json::json_string2(p) {
        Ok((value, encoded)) => {
            //let encoded = p.buffer[start.cursor..p.state.cursor].iter().collect();
            return Ok(HurlString {
                value,
                encoded: Some(encoded),
                source_info: SourceInfo {
                    start: start.pos,
                    end: p.clone().state.pos,
                },
            });
        }
        Err(e) => {
            if e.recoverable {
                let value =
                    p.next_chars_while(|c| c.is_alphanumeric() | vec!['_', '-', '.'].contains(&c));
                return Ok(HurlString {
                    value: value.clone(),
                    encoded: None,
                    source_info: SourceInfo {
                        start: start.pos,
                        end: p.clone().state.pos,
                    },
                });
            } else {
                Err(e)
            }
        }
    };
}

#[test]
fn test_name() {
    let mut parser = Parser::init("Foo");
    assert_eq!(
        name(&mut parser).unwrap(),
        HurlString {
            value: String::from("Foo"),
            encoded: None,
            source_info: SourceInfo::init(1, 1, 1, 4),
        }
    );
}

#[test]
fn test_name_json() {
    // "\u0046oo"
    let mut parser = Parser::init("\"\\u0046oo\"");
    assert_eq!(
        name(&mut parser).unwrap(),
        HurlString {
            value: String::from("Foo"),
            encoded: Some(String::from("\\u0046oo")),
            source_info: SourceInfo::init(1, 1, 1, 11),
        }
    );
}

#[test]
fn test_name_error() {
    let mut parser = Parser::init("\"Foo");
    let error = name(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 5 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("\"") });
    println!("{:?}", error);
}
// endregion

// region hurl-value

#[allow(dead_code)]
pub fn hurl_value(p: &mut Parser) -> ParseResult<'static, HurlTemplate> {
    return choice(vec![hurl_value_json, hurl_value_text], p);
}

#[test]
fn test_hurl_value() {
    let mut parser = Parser::init("café");
    assert_eq!(
        hurl_value(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("café"),
                    encoded: None,
                }
            }],
            delimiter: "".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 5),
        }
    );

    let mut parser = Parser::init("\"Bar{{name}}\"");
    assert_eq!(
        hurl_value(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![
                HurlTemplateElement::Literal {
                    value: HurlString2 {
                        value: String::from("Bar"),
                        encoded: Some(String::from("Bar")),
                    }
                },
                HurlTemplateElement::Expression {
                    value: Expr {
                        space0: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(1, 7, 1, 7),
                        },
                        variable: Variable {
                            name: String::from("name"),
                            source_info: SourceInfo::init(1, 7, 1, 11),
                        },
                        space1: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(1, 11, 1, 11),
                        },
                    }
                }
            ],
            //encoded: None,
            delimiter: "\"".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 14),
        }
    );
}

// endregion

// region hurl_value_json
pub fn hurl_value_json(p: &mut Parser) -> ParseResult<'static, HurlTemplate> {
    let start = p.state.clone();

    try_literal("\"", p)?;
    let mut elements = vec![];
    let mut buffer = String::from("");
    let mut encoded = String::from("");

    loop {
        let save = p.clone().state;

        match expr::parse(p) {
            Ok(value) => {
                if !buffer.is_empty() {
                    elements.push(HurlTemplateElement::Literal {
                        value: HurlString2 {
                            value: buffer.clone(),
                            encoded: Some(encoded.clone()),
                        },
                    });
                    buffer = String::from("");
                    encoded = String::from("");
                }
                elements.push(HurlTemplateElement::Expression { value });
            }
            Err(e) => {
                if !e.recoverable {
                    return Err(e);
                } else {
                    p.state = save.clone();
                    match json::json_char(p) {
                        Err(e) => return Err(e),
                        Ok(None) => {
                            break;
                        }
                        Ok(Some((c, s))) => {
                            buffer.push(c);
                            encoded.push_str(s.as_str());
                        }
                    }
                }
            }
        }
    }
    literal("\"", p)?;
    if !buffer.is_empty() {
        elements.push(HurlTemplateElement::Literal {
            value: HurlString2 {
                value: buffer.clone(),
                encoded: Some(encoded.clone()),
            },
        });
    }
    return Ok(HurlTemplate {
        elements,
        delimiter: "\"".to_string(),
        source_info: SourceInfo {
            start: start.pos,
            end: p.clone().state.pos,
        },
    });
}

#[test]
fn test_hurl_value_json() {
    let mut parser = Parser::init("\"\"");
    assert_eq!(
        hurl_value(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![],
            source_info: SourceInfo::init(1, 1, 1, 3),
            delimiter: "\"".to_string(),
        }
    );
    let mut parser = Parser::init("\"B\\u0061r\"");
    assert_eq!(
        hurl_value(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("Bar"),
                    encoded: Some(String::from("B\\u0061r")),
                }
            }],
            source_info: SourceInfo::init(1, 1, 1, 11),
            delimiter: "\"".to_string(),
        }
    );
}

#[test]
fn test_hurl_value_json_with_variable() {
    // "H\u0065llo {{\u0020n\u0061me}}!"
    let mut parser = Parser::init("\"H\\u0065llo {{ name}}!\"");
    assert_eq!(
        hurl_value(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![
                HurlTemplateElement::Literal {
                    value: HurlString2 {
                        value: String::from("Hello "),
                        encoded: Some(String::from("H\\u0065llo ")),
                    }
                },
                HurlTemplateElement::Expression {
                    value: Expr {
                        space0: Whitespace {
                            value: String::from(" "),
                            source_info: SourceInfo::init(1, 15, 1, 16),
                        },
                        variable: Variable {
                            name: String::from("name"),
                            source_info: SourceInfo::init(1, 16, 1, 20),
                        },
                        space1: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(1, 20, 1, 20),
                        },
                    }
                },
                HurlTemplateElement::Literal {
                    value: HurlString2 {
                        value: String::from("!"),
                        encoded: Some(String::from("!")),
                    }
                },
            ],
            delimiter: "\"".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 24),
        }
    );
}

#[test]
fn test_hurl_value_json_with_error() {
    let mut parser = Parser::init("\"H\\u0065llo {{name>}}!\"");
    let error = hurl_value(&mut parser).err().unwrap();
    assert_eq!(
        error.pos,
        Pos {
            line: 1,
            column: 19,
        }
    );
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("}}") });
}

#[test]
fn test_hurl_value_json_with_error2() {
    let mut parser = Parser::init("\"{{url\"");
    let error = hurl_value(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 7 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("}}") });
}
// endregion

// region hurl_value_text
// can be empty
fn hurl_value_text(p: &mut Parser) -> ParseResult<'static, HurlTemplate> {
    let start = p.clone().state;
    let mut elements = vec![];
    let mut buffer = String::from("");

    loop {
        let save = p.clone().state;
        match line_terminator(p) {
            Ok(_) => {
                p.state = save.clone();
                break;
            }
            _ => p.state = save.clone(),
        }

        match expr::parse(p) {
            Ok(value) => {
                if !buffer.is_empty() {
                    elements.push(HurlTemplateElement::Literal {
                        value: HurlString2 {
                            value: buffer.clone(),
                            encoded: None,
                        },
                    });
                    buffer = String::from("");
                }
                elements.push(HurlTemplateElement::Expression { value });
            }
            Err(e) => {
                if !e.recoverable {
                    return Err(e);
                } else {
                    p.state = save.clone();
                    match p.next_char() {
                        None => break,
                        Some(c) => {
                            buffer.push(c);
                        }
                    }
                }
            }
        }
    }
    if !buffer.is_empty() {
        elements.push(HurlTemplateElement::Literal {
            value: HurlString2 {
                value: buffer,
                encoded: None,
            },
        });
    }

    //    if elements.is_empty() {
    //        elements.push(HurlTemplateElement::Literal { value: HurlString2 { value: String::from(""), encoded: None }  });
    //    }
    return Ok(HurlTemplate {
        elements,
        delimiter: "".to_string(),
        source_info: SourceInfo {
            start: start.pos,
            end: p.state.clone().pos,
        },
    });
}

#[test]
fn test_hurl_value_text() {
    let mut parser = Parser::init(" # empty value");
    assert_eq!(
        hurl_value_text(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![],
            source_info: SourceInfo::init(1, 1, 1, 1),
            delimiter: "".to_string(),
        }
    );

    let mut parser = Parser::init("Bar");
    assert_eq!(
        hurl_value_text(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("Bar"),
                    encoded: None,
                }
            }],
            source_info: SourceInfo::init(1, 1, 1, 4),
            delimiter: "".to_string(),
        }
    );
}

#[test]
fn test_hurl_value_text_with_variable() {
    let mut parser = Parser::init("{{name}}");
    assert_eq!(
        hurl_value_text(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![HurlTemplateElement::Expression {
                value: Expr {
                    space0: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(1, 3, 1, 3),
                    },
                    variable: Variable {
                        name: String::from("name"),
                        source_info: SourceInfo::init(1, 3, 1, 7),
                    },
                    space1: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(1, 7, 1, 7),
                    },
                }
            }],
            source_info: SourceInfo::init(1, 1, 1, 9),
            delimiter: "".to_string(),
        }
    );
}

// endregion

// region filename

pub fn filename(p: &mut Parser) -> ParseResult<'static, Filename> {
    let start = p.state.clone();
    let s = p.next_chars_while(|c| c.is_alphanumeric() || *c == '.' || *c == '/' || *c == '_');
    if s.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Filename {},
        });
    }
    return Ok(Filename {
        value: s,
        source_info: SourceInfo {
            start: start.pos,
            end: p.state.clone().pos,
        },
    });
}

#[test]
fn test_filename() {
    let mut parser = Parser::init("???");
    let error = filename(&mut parser).err().unwrap();
    assert_eq!(error.inner, ParseError::Filename {});
    assert_eq!(error.pos, Pos { line: 1, column: 1 });

    let mut parser = Parser::init("/tmp/data.bin");
    assert_eq!(filename(&mut parser).unwrap(),
               Filename {
                   value: String::from("/tmp/data.bin"),
                   source_info: SourceInfo::init(1, 1, 1, 14),
               }
    );
}

// endregion

// region boolean
// no "separator" => always recoverable
pub fn boolean(p: &mut Parser) -> ParseResult<'static, bool> {
    let start = p.state.clone();
    return match try_literal("true", p) {
        Ok(_) => Ok(true),
        Err(_) => match literal("false", p) {
            Ok(_) => Ok(false),
            Err(_) => Err(Error {
                pos: start.pos,
                recoverable: true,
                inner: ParseError::Expecting {value: String::from("true|false")},
            }),
        },
    };
}

#[test]
fn test_boolean() {
    let mut parser = Parser::init("true");
    assert_eq!(boolean(&mut parser).unwrap(), true);

    let mut parser = Parser::init("xxx");
    let error = boolean(&mut parser).err().unwrap();
    assert_eq!(error.inner, ParseError::Expecting {value: String::from("true|false")});
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("trux");
    let error = boolean(&mut parser).err().unwrap();
    assert_eq!(error.inner, ParseError::Expecting {value: String::from("true|false")});
    assert_eq!(error.recoverable, true);
}

// endregion

// region natural

pub fn natural(p: &mut Parser) -> ParseResult<'static, u64> {
    let start = p.state.clone();

    if p.is_eof() {
        return Err(Error {
            pos: start.pos,
            recoverable: true,
            inner: ParseError::Expecting { value: String::from("natural") },
        });
    }
    let first_digit = p.next_char().unwrap();
    if !first_digit.is_digit(10) {
        return Err(Error {
            pos: start.pos,
            recoverable: true,
            inner: ParseError::Expecting { value: String::from("natural") },
        });
    }

    let save = p.state.clone();
    let s = p.next_chars_while(|c| c.is_digit(10));

    // if the first digit is zero, you should not have any more digits
    if first_digit == '0' && s.len() > 0 {
        return Err(Error {
            pos: save.pos,
            recoverable: false,
            inner: ParseError::Expecting { value: String::from("natural") },
        });
    }
    return Ok(format!("{}{}", first_digit, s).parse().unwrap());
}

#[test]
fn test_natural() {
    let mut parser = Parser::init("0");
    assert_eq!(natural(&mut parser).unwrap(), 0);
    assert_eq!(parser.state.cursor, 1);

    let mut parser = Parser::init("0.");
    assert_eq!(natural(&mut parser).unwrap(), 0);
    assert_eq!(parser.state.cursor, 1);

    let mut parser = Parser::init("10x");
    assert_eq!(natural(&mut parser).unwrap(), 10);
    assert_eq!(parser.state.cursor, 2);
}

#[test]
fn test_natural_error() {
    let mut parser = Parser::init("");
    let error = natural(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("natural") });
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("01");
    let error = natural(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 2 });
    assert_eq!(error.inner,ParseError::Expecting { value: String::from("natural") });
    assert_eq!(error.recoverable, false);

    let mut parser = Parser::init("x");
    let error = natural(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner,ParseError::Expecting { value: String::from("natural") });
    assert_eq!(error.recoverable, true);
}
// endregion

// region integer
// recoverable

pub fn integer(p: &mut Parser) -> ParseResult<'static, i64> {
    let sign = match try_literal("-", p) {
        Err(_) => 1,
        Ok(_) => -1,
    };
    let nat = natural(p)?;
    return Ok(sign * (nat as i64));
}

#[test]
pub fn test_integer() {
    let mut parser = Parser::init("1");
    assert_eq!(integer(&mut parser).unwrap(), 1);

    let mut parser = Parser::init("1.1");
    assert_eq!(integer(&mut parser).unwrap(), 1);

    let mut parser = Parser::init("-1.1");
    assert_eq!(integer(&mut parser).unwrap(), -1);

    let mut parser = Parser::init("x");
    let error = integer(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("natural") });
    assert_eq!(error.recoverable, true);
}

// endregion

// region float
// non recoverable after the dot


// an integer is parsed ok as float => no like a computer language
pub fn float(p: &mut Parser) -> ParseResult<'static, Float> {
    let int = integer(p)?;

    try_literal(".", p)?;

    if p.is_eof() {
        return Err(Error {
            pos: p.clone().state.pos,
            recoverable: false,
            inner: ParseError::Expecting {value: String::from("natural")},
        });
    }

    let s = p.next_chars_while(|c| c.is_digit(10));
    if s.len() == 0 {
        return Err(Error {
            pos: p.clone().state.pos,
            recoverable: false,
            inner: ParseError::Expecting {value: String::from("natural")},
        });
    }
    let decimal = format!("{:0<18}", s).parse().unwrap();
    let decimal_digits = s.len();
    return Ok(Float { int, decimal, decimal_digits });
}

#[test]
fn test_float() {
    let mut parser = Parser::init("1.0");
    assert_eq!(float(&mut parser).unwrap(), Float { int: 1, decimal: 0, decimal_digits: 1 });
    assert_eq!(parser.state.cursor, 3);

    let mut parser = Parser::init("-1.0");
    assert_eq!(float(&mut parser).unwrap(), Float { int: -1, decimal: 0, decimal_digits: 1 });
    assert_eq!(parser.state.cursor, 4);

    let mut parser = Parser::init("1.1");
    assert_eq!(float(&mut parser).unwrap(), Float { int: 1, decimal: 100000000000000000, decimal_digits: 1 });
    assert_eq!(parser.state.cursor, 3);

    let mut parser = Parser::init("1.100");
    assert_eq!(float(&mut parser).unwrap(), Float { int: 1, decimal: 100000000000000000, decimal_digits: 3 });
    assert_eq!(parser.state.cursor, 5);

    let mut parser = Parser::init("1.01");
    assert_eq!(float(&mut parser).unwrap(), Float { int: 1, decimal: 10000000000000000, decimal_digits: 2 });
    assert_eq!(parser.state.cursor, 4);

    let mut parser = Parser::init("1.010");
    assert_eq!(float(&mut parser).unwrap(), Float { int: 1, decimal: 10000000000000000, decimal_digits: 3 });
    assert_eq!(parser.state.cursor, 5);

    let mut parser = Parser::init("-0.333333333333333333");
    assert_eq!(float(&mut parser).unwrap(), Float { int: 0, decimal: 333333333333333333, decimal_digits: 18 });
    assert_eq!(parser.state.cursor, 21);
}

#[test]
fn test_float_error() {
    let mut parser = Parser::init("");
    let error = float(&mut parser).err().unwrap();
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("natural") });
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("-");
    let error = float(&mut parser).err().unwrap();
    assert_eq!(error.inner,ParseError::Expecting { value: String::from("natural") });
    assert_eq!(error.pos, Pos { line: 1, column: 2 });
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("1");
    let error = float(&mut parser).err().unwrap();
    assert_eq!(error.inner, ParseError::Expecting { value: String::from(".") });
    assert_eq!(error.pos, Pos { line: 1, column: 2 });
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("1x");
    let error = float(&mut parser).err().unwrap();
    assert_eq!(error.inner, ParseError::Expecting { value: String::from(".") });
    assert_eq!(error.pos, Pos { line: 1, column: 2 });
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("1.");
    let error = float(&mut parser).err().unwrap();
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("natural") });
    assert_eq!(error.pos, Pos { line: 1, column: 3 });
    assert_eq!(error.recoverable, false);

    let mut parser = Parser::init("1.x");
    let error = float(&mut parser).err().unwrap();
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("natural") });
    assert_eq!(error.pos, Pos { line: 1, column: 3 });
    assert_eq!(error.recoverable, false);
}


// endregion

//// region line
//// TBC => consume or not the end of line
//pub fn line(p: &mut Parser) -> ParseResult<'static, String> {
//    let mut s = String::from("");
//    if p.clone().is_eof() {
//        return Err(Error {
//            pos: p.clone().state.pos,
//            recoverable: false,
//            inner: ParseError::Eof {},
//        });
//    }
//    loop {
//        //let save = p.state.clone();
//        if p.remaining().starts_with("\n") {
//            return Ok(s);
//        }
//        match p.next_char() {
//            None => {
//                return Ok(s);
//            }
//            Some(c) => {
//                s.push(c);
//            }
//        }
//    }
//}
//
//
//#[test]
//fn test_line() {
//    let mut parser = Parser::init("");
//    let error = line(&mut parser).err().unwrap();
//    assert_eq!(error.pos, Pos { line: 1, column: 1 });
//    assert_eq!(error.inner, ParseError::Eof {});
//
//    let mut parser = Parser::init("xxx");
//    assert_eq!(line(&mut parser).unwrap(), "xxx");
//
//    let mut parser = Parser::init("xxx\nxxx");
//    assert_eq!(line(&mut parser).unwrap(), "xxx");
//}
//// endregion

// region bytes

pub fn bytes(p: &mut Parser) -> ParseResult<'static, Bytes> {
    //let start = p.state.clone();
    return choice(vec![raw_string, json_bytes, xml_bytes, base64_bytes, file_bytes], p);
}

#[test]
fn test_bytes_json() {
    let mut parser = Parser::init("{ } ");
    assert_eq!(
        bytes(&mut parser).unwrap(),
        Bytes::Json {
            value: String::from("{ }")
        }
    );
    assert_eq!(parser.state.cursor, 3);

    let mut parser = Parser::init("true");
    assert_eq!(
        bytes(&mut parser).unwrap(),
        Bytes::Json {
            value: String::from("true")
        }
    );
    assert_eq!(parser.state.cursor, 4);

    let mut parser = Parser::init("\"\" x");
    assert_eq!(
        bytes(&mut parser).unwrap(),
        Bytes::Json {
            value: String::from("\"\"")
        }
    );
    assert_eq!(parser.state.cursor, 2);
}

#[test]
fn test_bytes_xml() {
    let mut parser = Parser::init("<a/>");
    assert_eq!(
        bytes(&mut parser).unwrap(),
        Bytes::Xml {
            value: String::from("<a/>")
        }
    );
}

#[test]
fn test_bytes_file() {
    let mut parser = Parser::init("file,data.xml;");
    assert_eq!(
        bytes(&mut parser).unwrap(),
        Bytes::File {
            space0: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(1, 6, 1, 6),
            },
            filename: Filename {
                value: String::from("data.xml"),
                source_info: SourceInfo::init(1, 6, 1, 14),
            },
            space1: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(1, 14, 1, 14),
            },
        }
    );
}

#[test]
fn test_bytes_json_error() {
    let mut parser = Parser::init("{ x ");
    let error = bytes(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 2 });
    assert_eq!(error.inner, ParseError::Json {});
}

#[test]
fn test_bytes_multilines_error() {
    let mut parser = Parser::init("```\nxxx ");
    let error = bytes(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 2, column: 5 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("```")});
}

#[test]
fn test_bytes_eof() {
    let mut parser = Parser::init("");
    let error = bytes(&mut parser).err().unwrap();
    //println!("{:?}", error);
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("file")});
    assert_eq!(error.recoverable, true);
}
// endregion

// region xml-bytes
pub fn xml_bytes(p: &mut Parser) -> ParseResult<'static, Bytes> {
    return match xml::parse(p) {
        Err(e) => Err(e),
        Ok(value) => Ok(Bytes::Xml { value }),
    };
}
//endregion

// region json-bytes
pub fn json_bytes(p: &mut Parser) -> ParseResult<'static, Bytes> {
    return match json::json_value(p) {
        Err(e) => Err(e),
        Ok(value) => Ok(Bytes::Json { value }),
    };
}
// endregion

// region raw-string
// one value without newline or multiline mode
// includes the last newline (consistent with bash EOL)
pub fn raw_string(p: &mut Parser) -> ParseResult<'static, Bytes> {
    try_literal("```", p)?;
    let save = p.state.clone();
    match newline(p) {
        Ok(newline0) => {
            let value = raw_string_multilines(p)?;
            return Ok(Bytes::MultilineString { newline0, value });
        }
        Err(_) => {
            p.state = save;
            let newline0 = Whitespace {
                value: String::from(""),
                source_info: SourceInfo {
                    start: p.state.clone().pos,
                    end: p.state.clone().pos,
                },
            };
            let value = raw_string_value(p)?;
            return Ok(Bytes::MultilineString { newline0, value });
        }
    }
}


#[test]
fn test_multiline_string_empty() {
    let mut parser = Parser::init("``````");
    assert_eq!(raw_string(&mut parser).unwrap(), Bytes::MultilineString {
        newline0: Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(1, 4, 1, 4),
        },
        value: String::from(""),
    });

    let mut parser = Parser::init("```\n```");
    assert_eq!(raw_string(&mut parser).unwrap(), Bytes::MultilineString {
        newline0: Whitespace {
            value: String::from("\n"),
            source_info: SourceInfo::init(1, 4, 2, 1),
        },
        value: String::from(""),
    });
    let mut parser = Parser::init("```\r\n```");
    assert_eq!(raw_string(&mut parser).unwrap(), Bytes::MultilineString {
        newline0: Whitespace {
            value: String::from("\r\n"),
            source_info: SourceInfo::init(1, 4, 2, 1),
        },
        value: String::from(""),
    });
}

#[test]
fn test_multiline_string_hello() {
    let mut parser = Parser::init("```Hello World!```");
    assert_eq!(raw_string(&mut parser).unwrap(), Bytes::MultilineString {
        newline0: Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(1, 4, 1, 4),
        },
        value: String::from("Hello World!"),
    });
    let mut parser = Parser::init("```Hello\nWorld!\n```");
    assert_eq!(raw_string(&mut parser).unwrap(), Bytes::MultilineString {
        newline0: Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(1, 4, 1, 4),
        },
        value: String::from("Hello\nWorld!\n"),
    });
}


#[test]
fn test_multiline_string_csv() {
    let mut parser = Parser::init("```\nline1\nline2\nline3\n```");
    assert_eq!(raw_string(&mut parser).unwrap(), Bytes::MultilineString {
        newline0: Whitespace {
            value: String::from("\n"),
            source_info: SourceInfo::init(1, 4, 2, 1),
        },
        value: String::from("line1\nline2\nline3\n"),
    });
}

#[test]
fn test_multiline_string_one_emptyline() {


    // one newline
    // the value takes the value of the newline??
    let mut parser = Parser::init("```\n\n```");
    assert_eq!(
        raw_string(&mut parser).unwrap(),
        Bytes::MultilineString {
            newline0: Whitespace {
                value: String::from("\n"),
                source_info: SourceInfo::init(1, 4, 2, 1),
            },
            value: String::from("\n"),
        }
    );

    // one cr
    let mut parser = Parser::init("```\n\r\n````");
    assert_eq!(
        raw_string(&mut parser).unwrap(),
        Bytes::MultilineString {
            newline0: Whitespace {
                value: String::from("\n"),
                source_info: SourceInfo::init(1, 4, 2, 1),
            },
            value: String::from("\r\n"),
        }
    );
}

#[test]
fn test_raw_string_error() {
    let mut parser = Parser::init("xxx");
    let error = raw_string(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("```") });
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("```\nxxx");
    let error = raw_string(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 2, column: 4 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("```")});
    assert_eq!(error.recoverable, false);

    let mut parser = Parser::init("```xxx");
    let error = raw_string(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 7 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("```")});
    assert_eq!(error.recoverable, false);
}
// endregion

// region raw-string-multilines

pub fn raw_string_multilines(p: &mut Parser) -> ParseResult<'static, String> {
    let mut value = String::from("");
    loop {
        let save = p.state.clone();
        match literal("```", p) {
            Ok(_) => {
                break ();
            }
            Err(_) => {}
        }
        p.state = save;
        match p.next_char() {
            None => {
                return Err(Error {
                    pos: p.clone().state.pos,
                    recoverable: false,
                    inner: ParseError::Expecting {value: String::from("```")},
                });
            }
            Some(c) => {
                value.push(c);
            }
        }
    }
    return Ok(value);
}

#[test]
fn test_raw_string_multilines() {
    let mut parser = Parser::init("```");
    assert_eq!(
        raw_string_multilines(&mut parser).unwrap(),
        String::from("")
    );
    assert_eq!(parser.state.cursor, 3);

    let mut parser = Parser::init("line1\n```");
    assert_eq!(
        raw_string_multilines(&mut parser).unwrap(),
        String::from("line1\n")
    );
    assert_eq!(parser.state.cursor, 9);
}
// endregion

// region raw-string-value
pub fn raw_string_value(p: &mut Parser) -> ParseResult<'static, String> {
    let mut value = String::from("");
    loop {
        let save = p.state.clone();
        match literal("```", p) {
            Ok(_) => break,
            Err(_) => {
                p.state = save;
                let _save = p.state.clone();
                match p.next_char() {
                    None => {
                        return Err(Error {
                            pos: p.clone().state.pos,
                            recoverable: false,
                            inner: ParseError::Expecting {value:String::from("```")},
                        });
                    }
//                    Some('\n') => {
//                        return Err(Error {
//                            pos: save.pos,
//                            recoverable: false,
//                            inner: ParseError::Unexpected {
//                                character: String::from("newline"),
//                            },
//                        });
//                    }
                    Some(c) => {
                        value.push(c);
                    }
                }
            }
        };
    }
    return Ok(value);
}

#[test]
fn test_raw_string_value() {
    let mut parser = Parser::init("```");
    assert_eq!(raw_string_value(&mut parser).unwrap(), String::from(""));
    assert_eq!(parser.state.cursor, 3);

    let mut parser = Parser::init("hello```");
    assert_eq!(
        raw_string_value(&mut parser).unwrap(),
        String::from("hello")
    );
    assert_eq!(parser.state.cursor, 8);
}
// endregion

// region file-bytes

pub fn file_bytes(p: &mut Parser) -> ParseResult<'static, Bytes> {
    let _start = p.state.clone();
    try_literal("file", p)?;
    literal(",", p)?;
    let space0 = zero_or_more_spaces(p)?;
    let f = filename(p)?;
    let space1 = zero_or_more_spaces(p)?;
    literal(";", p)?;
    return Ok(Bytes::File {
        space0,
        filename: f,
        space1,
    });
}

#[test]
fn test_file_bytes() {
    let mut parser = Parser::init("file, filename1;");
    assert_eq!(
        file_bytes(&mut parser).unwrap(),
        Bytes::File {
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 6, 1, 7),
            },
            filename: Filename {
                value: String::from("filename1"),
                source_info: SourceInfo::init(1, 7, 1, 16),
            },
            space1: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(1, 16, 1, 16),
            },
        }
    );

    let mut parser = Parser::init("file, /tmp/filename1;");
    assert_eq!(
        file_bytes(&mut parser).unwrap(),
        Bytes::File {
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 6, 1, 7),
            },
            filename: Filename {
                value: String::from("/tmp/filename1"),
                source_info: SourceInfo::init(1, 7, 1, 21),
            },
            space1: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(1, 21, 1, 21),
            },
        }
    );
}

#[test]
fn test_file_bytes_error() {
    let mut parser = Parser::init("fil; filename1;");
    let error = file_bytes(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("file, filename1");
    let error = file_bytes(&mut parser).err().unwrap();
    assert_eq!(
        error.pos,
        Pos {
            line: 1,
            column: 16,
        }
    );
    assert_eq!(error.recoverable, false);
    assert_eq!(error.inner, ParseError::Expecting { value: String::from(";") });
}
// endregion

// region base64-bytes

// base64 => can have whitespace
// support pqrser position

pub fn base64_bytes(p: &mut Parser) -> ParseResult<'static, Bytes> {
    let _start = p.state.clone();
    try_literal("base64", p)?;
    literal(",", p)?;
    let space0 = zero_or_more_spaces(p)?;
    let save_state = p.state.clone();
    let value = base64::parse2(p);
    let count = p.state.cursor - save_state.cursor;
    p.state = save_state;
    let encoded = p.next_chars(count).unwrap();
    let space1 = zero_or_more_spaces(p)?;
    literal(";", p)?;
    return Ok(Bytes::Base64 {
        space0,
        value,
        encoded,
        space1,
    });
}

#[test]
fn test_base64_bytes() {
    let mut parser = Parser::init("base64,  T WE=;xxx");
    assert_eq!(
        base64_bytes(&mut parser).unwrap(),
        Bytes::Base64 {
            space0: Whitespace {
                value: String::from("  "),
                source_info: SourceInfo::init(1, 8, 1, 10),
            },
            value: vec![77, 97],
            encoded: String::from("T WE="),
            space1: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(1, 15, 1, 15),
            },
        }
    );
    assert_eq!(parser.state.cursor, 15);
}

// endregion

// region eof
pub fn eof(p: &mut Parser) -> ParseResult<'static, ()> {
    return if p.is_eof() {
        Ok(())
    } else {
        Err(Error {
            pos: p.state.clone().pos,
            recoverable: false,
            inner: ParseError::Expecting {value: String::from("eof")},
        })
    };
}
// endregion



