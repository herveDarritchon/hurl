use sxd_document::parser;

use crate::core::core::Pos;
use super::core::{Parser, ParseResult};
use super::error::{Error, ParseError};




fn is_valid(s: &str) -> bool {
    return match parser::parse(s) {
        Ok(_) => { true },
        _ => false
    };
}

pub fn parse(p: &mut Parser) -> ParseResult<'static, String> {
    let mut buf = String::from("");
    let start = p.state.clone();
    match p.next_char() {
        Some('<') => {  buf.push('<')},
        _ =>  return Err(Error{
            pos: Pos {line: 1, column: 1},
            recoverable: true,
            inner: ParseError::Xml{}
        })
    }

    loop {
        match p.next_char() {
            None => { break;}
            Some(c) => {
                buf.push(c);
                if c == '>' && is_valid(buf.as_str()) {
                    return Ok(buf.clone());
                }
            }
        }
    }
    return Err(Error{
        pos: start.pos,
        recoverable: false,
        inner: ParseError::Xml{}
    });
}


#[test]
fn test_parsing_xml_brute_force_errors() {

    let mut parser = Parser::init("");
    let error = parse(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos{ line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Xml{});
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("x");
    let error = parse(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos{ line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Xml{});
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("<<");
    let error = parse(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos{ line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Xml{});
    assert_eq!(error.recoverable, false);

    let mut parser = Parser::init("<users><user /></users");
    let error = parse(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos{ line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Xml{});

    let mut parser = Parser::init("<users aa><user /></users");
    let error = parse(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos{ line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Xml{});


}

#[test]
fn test_parsing_xml_brute_force() {

    let mut parser = Parser::init("<users><user /></users>");
    assert_eq!(parse(&mut parser).unwrap(), String::from("<users><user /></users>"));
    assert_eq!(parser.state.cursor, 23);

    let mut parser = Parser::init("<users><user /></users>xx");
    assert_eq!(parse(&mut parser).unwrap(), String::from("<users><user /></users>"));
    assert_eq!(parser.state.cursor, 23);
    assert_eq!(parser.remaining(), String::from("xx"));

}