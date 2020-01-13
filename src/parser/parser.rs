use crate::core::ast::*;
use crate::core::core::Pos;
use crate::core::core::SourceInfo;

use super::combinators::*;
use super::core::*;
use super::error::*;
use super::expr;
use super::json;
use super::primitives::*;

// region hurl-file
pub fn hurl_file(p: &mut Parser) -> ParseResult<'static, HurlFile> {
    let entries = zero_or_more(|p1| entry(p1), p)?;
    let line_terminators = optional_line_terminators(p)?;
    eof(p)?;
    return Ok(HurlFile {
        entries,
        line_terminators,
    });
}

#[test]
fn test_hurl_file() {
    let mut parser = Parser::init("GET http://google.fr");
    let hurl_file = hurl_file(&mut parser).unwrap();

    assert_eq!(hurl_file.entries.len(), 1);
}
// endregion

// region entry
pub fn entry(p: &mut Parser) -> ParseResult<'static, Entry> {
    let req = request(p)?;
    let resp = optional(|p1| response(p1), p)?;
    return Ok(Entry {
        request: req,
        response: resp,
    });
}

#[test]
fn test_entry() {
    let mut parser = Parser::init("GET http://google.fr");
    //println!("{:?}", entry(&mut parser));
    let e = entry(&mut parser).unwrap();
    assert_eq!(e.request.method, Method::Get);
    assert_eq!(parser.state.cursor, 20);
}

#[test]
fn test_several_entry() {
    let mut parser = Parser::init("GET http://google.fr\nGET http://google.fr");

    let e = entry(&mut parser).unwrap();
    //println!("{:?}", e);
    assert_eq!(e.request.method, Method::Get);
    assert_eq!(parser.state.cursor, 21);
    assert_eq!(parser.state.pos.line, 2);

    let e = entry(&mut parser).unwrap();
    assert_eq!(e.request.method, Method::Get);
    assert_eq!(parser.state.cursor, 41);
    assert_eq!(parser.state.pos.line, 2);

    let mut parser =
        Parser::init("GET http://google.fr # comment1\nGET http://google.fr # comment2");

    let e = entry(&mut parser).unwrap();
    assert_eq!(e.request.method, Method::Get);
    assert_eq!(parser.state.cursor, 32);
    assert_eq!(parser.state.pos.line, 2);

    let e = entry(&mut parser).unwrap();
    assert_eq!(e.request.method, Method::Get);
    assert_eq!(parser.state.cursor, 63);
    assert_eq!(parser.state.pos.line, 2);
}

#[test]
fn test_entry_with_response() {
    let mut parser = Parser::init("GET http://google.fr\nHTTP/1.1 200");
    let e = entry(&mut parser).unwrap();
    assert_eq!(e.request.method, Method::Get);
    assert_eq!(e.response.unwrap().status.value, 200);
}

// endregion

// region request
pub fn request(p: &mut Parser) -> ParseResult<'static, Request> {
    let start = p.state.clone();
    let line_terminators = optional_line_terminators(p)?;
    let space0 = zero_or_more_spaces(p)?;
    let m = method(p)?;
    let space1 = one_or_more_spaces(p)?;
    let u = url(p)?;
    let line_terminator0 = line_terminator(p)?;
    let headers = zero_or_more(header, p)?;
    let sections = request_sections(p)?;
    let b = optional(|p1| body(p1), p)?;
    let source_info = SourceInfo::init(
        start.pos.line,
        start.pos.column,
        p.state.pos.line,
        p.state.pos.column,
    );
    return Ok(Request {
        line_terminators,
        space0,
        method: m,
        space1,
        url: u,
        line_terminator0,
        headers,
        sections,
        body: b,
        source_info,
    });
}

//#[test]
//fn test_request() {
//    let mut parser = Parser::init("GET http://google.fr");
//    let default_request = Request {
//        line_terminators: vec![],
//        space0: Whitespace {
//            value: "".to_string(),
//            source_info: SourceInfo::init(1, 1, 1, 1),
//        },
//        method: Method::Get,
//        space1: Whitespace {
//            value: " ".to_string(),
//            source_info: SourceInfo::init(1, 4, 1, 5),
//        },
//        url: HurlTemplate {
//            elements: vec![
//                HurlTemplateElement::Literal { value: HurlString2 { value: String::from("http://google.fr"), encoded: None } }
//            ],
//            delimiter: "".to_string(),
//            source_info: SourceInfo::init(1, 5, 1, 21),
//        },
//        line_terminator0: LineTerminator {
//            space0: Whitespace {
//                value: "".to_string(),
//                source_info: SourceInfo::init(1, 21, 1, 21),
//            },
//            comment: None,
//            newline: Whitespace {
//                value: "".to_string(),
//                source_info: SourceInfo::init(1, 21, 1, 21),
//            },
//        },
//        headers: vec![],
//        sections: vec![],
//        query_params: None,
//        form_params: None,
//        cookies: None,
//        body: None,
//        source_info: SourceInfo::init(1, 1, 1, 21),
//    };
//    assert_eq!(request(&mut parser), Ok(default_request));
//
//    let mut parser = Parser::init("GET  http://google.fr # comment");
//    let default_request = Request {
//        line_terminators: vec![],
//        space0: Whitespace {
//            value: "".to_string(),
//            source_info: SourceInfo::init(1, 1, 1, 1),
//        },
//        method: Method::Get,
//        space1: Whitespace {
//            value: "  ".to_string(),
//            source_info: SourceInfo::init(1, 4, 1, 6),
//        },
//        url: HurlTemplate {
//            elements: vec![
//                HurlTemplateElement::Literal { value: HurlString2 { value: String::from("http://google.fr"), encoded: None } }
//            ],
//            delimiter: "".to_string(),
//            source_info: SourceInfo::init(1, 6, 1, 22),
//        },
//        line_terminator0: LineTerminator {
//            space0: Whitespace {
//                value: " ".to_string(),
//                source_info: SourceInfo::init(1, 22, 1, 23),
//            },
//            comment: Some(Comment {
//                value: " comment".to_string(),
//            }),
//            newline: Whitespace {
//                value: "".to_string(),
//                source_info: SourceInfo::init(1, 32, 1, 32),
//            },
//        },
//        headers: vec![],
//        sections: vec![],
//        query_params: None,
//        form_params: None,
//        cookies: None,
//        body: None,
//        source_info: SourceInfo::init(1, 1, 1, 32),
//    };
//    assert_eq!(request(&mut parser), Ok(default_request));
//
//    let mut parser = Parser::init("GET http://google.fr\nGET http://google.fr");
//    let r = request(&mut parser);
//    assert_eq!(r.unwrap().method, Method::Get);
//    assert_eq!(parser.state.cursor, 21);
//    let r = request(&mut parser).unwrap();
//    assert_eq!(r.method, Method::Get);
//}

//#[test]
//fn test_request_with_section_params() {
//    let mut parser = Parser::init("GET http://google.fr\n[QueryStringParams]\nq: rust");
//    let req = request(&mut parser).unwrap();
//
//    let query_params_section = Section {
//        line_terminators: vec![],
//        space0: Whitespace {
//            value: "".to_string(),
//            source_info: SourceInfo::init(2, 1, 2, 1),
//        },
//        name: String::from("QueryStringParams"),
//        line_terminator0: LineTerminator {
//            space0: Whitespace {
//                value: "".to_string(),
//                source_info: SourceInfo::init(2, 20, 2, 20),
//            },
//            comment: None,
//            newline: Whitespace {
//                value: "\n".to_string(),
//                source_info: SourceInfo::init(2, 20, 3, 1),
//            },
//        },
//        items: vec![
//            Param {
//                line_terminators: vec![],
//                space0: Whitespace {
//                    value: "".to_string(),
//                    source_info: SourceInfo::init(3, 1, 3, 1),
//                },
//                name: HurlString {
//                    value: String::from("q"),
//                    encoded: None,
//                    source_info: SourceInfo::init(3, 1, 3, 2),
//                },
//                space1: Whitespace {
//                    value: "".to_string(),
//                    source_info: SourceInfo::init(3, 2, 3, 2),
//                },
//                space2: Whitespace {
//                    value: " ".to_string(),
//                    source_info: SourceInfo::init(3, 3, 3, 4),
//                },
//                value: HurlTemplate {
//                    elements: vec![
//                        HurlTemplateElement::Literal { value: HurlString2 { value: String::from("rust"), encoded: None } }
//                    ],
//                    //encoded: None,
//                    delimiter: "".to_string(),
//                    source_info: SourceInfo::init(3, 4, 3, 8),
//                },
//                line_terminator0: LineTerminator {
//                    space0: Whitespace {
//                        value: "".to_string(),
//                        source_info: SourceInfo::init(3, 8, 3, 8),
//                    },
//                    comment: None,
//                    newline: Whitespace {
//                        value: "".to_string(),
//                        source_info: SourceInfo::init(3, 8, 3, 8),
//                    },
//                },
//            }
//        ],
//    };
//    assert_eq!(req.query_params.unwrap(), query_params_section);
//}

//#[test]
//fn test_request_multilines() {
//    let mut parser = Parser::init("GET http://google.fr\n'''Hello World!'''");
//    let default_request = Request {
//        line_terminators: vec![],
//        space0: Whitespace {
//            value: "".to_string(),
//            source_info: SourceInfo::init(1, 1, 1, 1),
//        },
//        method: Method::Get,
//        space1: Whitespace {
//            value: " ".to_string(),
//            source_info: SourceInfo::init(1, 4, 1, 5),
//        },
//        url: HurlTemplate::literal("http://google.fr".to_string()),
//        line_terminator0: LineTerminator {
//            space0: Whitespace {
//                value: "".to_string(),
//                source_info: SourceInfo::init(1, 21, 1, 21),
//            },
//            comment: None,
//            newline: Whitespace {
//                value: "\n".to_string(),
//                source_info: SourceInfo::init(1, 21, 2, 1),
//            },
//        },
//        headers: vec![],
//        query_params: None,
//        form_params: None,
//        cookies: None,
//        body: Some(Body {
//            line_terminators: vec![],
//            space0: Whitespace { value: String::from(""), source_info: SourceInfo::init(2, 1, 2, 1) },
//            value: Bytes::MultilineString { value: String::from("Hello World!") },
//            line_terminator0: LineTerminator {
//                space0: Whitespace { value: String::from(""), source_info: SourceInfo::init(2, 19, 2, 19) },
//                comment: None,
//                newline: Whitespace { value: String::from(""), source_info: SourceInfo::init(2, 19, 2, 19) },
//            },
//        }),
//        source_info: SourceInfo::init(1, 1, 2, 19),
//    };
//    assert_eq!(request(&mut parser), Ok(default_request));
//}
#[test]
fn test_request_multilines() {
    // GET http://google.fr
    // ```
    // Hello World!
    // ```
    let mut parser = Parser::init("GET http://google.fr\n```\nHello World!\n```");
    let req = request(&mut parser).unwrap();
    assert_eq!(
        req.body.unwrap(),
        Body {
            line_terminators: vec![],
            space0: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(2, 1, 2, 1),
            },
            value: Bytes::MultilineString {
                newline0: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::init(2, 4, 3, 1),
                },
                value: String::from("Hello World!\n"),
            },
            line_terminator0: LineTerminator {
                space0: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(4, 4, 4, 4),
                },
                comment: None,
                newline: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(4, 4, 4, 4),
                },
            },
        }
    );
}

//#[test]
//fn test_request_with_headers() {
//    let mut parser = Parser::init("GET http://google.fr\nFoo: Bar\n\n\"Fo o\": \"B ar\"");
//    let req = request(&mut parser).unwrap();
//
//    let header1 = req.headers.get(0).unwrap();
//    assert_eq!(header1.line_terminators.len(), 0);
//    assert_eq!(header1.name.value, "Foo");
//    assert_eq!(header1.value.elements.get(0).unwrap(),
//               &HurlTemplateElement::Literal  { value: HurlString { value: "Bar".to_string(), encoded: None, source_info: SourceInfo::init(0,0,0,0)  }});
//
//    let header2 = req.headers.get(1).unwrap();
//    assert_eq!(header2.line_terminators.len(), 1);
//    assert_eq!(header2.name.value, "Fo o");
//    assert_eq!(header2.value.elements.get(0).unwrap(),
//               &HurlTemplateElement::Literal  {
//                   value: HurlString {
//                       value: "B ar".to_string(),
//                       encoded: Some(String::from("B ar")),
//                       source_info: SourceInfo::init(0,0,0,0)
//                   }});
//
//    //println!("{:?}", );
//}

#[test]
fn test_request_error() {
    let mut parser = Parser::init("xxx");
    let error = request(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
}
// endregion

// region response

pub fn response(p: &mut Parser) -> ParseResult<'static, Response> {
    let start = p.state.clone();
    let line_terminators = optional_line_terminators(p)?;
    let space0 = zero_or_more_spaces(p)?;
    let _version = version(p)?;
    let space1 = one_or_more_spaces(p)?;
    let _status = status(p)?;
    let line_terminator0 = line_terminator(p)?;
    let headers = zero_or_more(header, p)?;
    let sections = response_sections(p)?;
    let b = optional(|p1| body(p1), p)?;
    return Ok(Response {
        line_terminators,
        space0,
        version: _version,
        space1,
        status: _status,
        line_terminator0,
        headers,
        sections,
        body: b,
        source_info: SourceInfo::init(
            start.pos.line,
            start.pos.column,
            p.state.pos.line,
            p.state.pos.column,
        ),
    });
}

#[test]
fn test_response() {
    let mut parser = Parser::init("HTTP/1.1 200");
    //println!("{:?}", response(&mut parser));
    let r = response(&mut parser).unwrap();

    assert_eq!(r.version.value, VersionValue::Version11);
    assert_eq!(r.status.value, 200);
}
// endregion

// region method
pub fn method(p: &mut Parser) -> ParseResult<'static, Method> {
    let start = p.state.clone();
    let available_methods = vec![
        ("GET", Method::Get),
        ("HEAD", Method::Head),
        ("POST", Method::Post),
        ("PUT", Method::Put),
        ("DELETE", Method::Delete),
        ("CONNECT", Method::Connect),
        ("OPTIONS", Method::Options),
        ("TRACE", Method::Trace),
    ];

    for (s, method) in available_methods {
        match try_literal(s, p) {
            Ok(_) => return Ok(method),
            _ => {}
        }
    }

    return Err(Error {
        pos: start.pos,
        recoverable: p.is_eof(),
        inner: ParseError::Method {},
    });
}

#[test]
fn test_method() {
    let mut parser = Parser::init("xxx ");
    let error = method(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(parser.state.cursor, 0);

    let mut parser = Parser::init("");
    let error = method(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(parser.state.cursor, 0);

    let mut parser = Parser::init("GET ");
    assert_eq!(Ok(Method::Get), method(&mut parser));
    assert_eq!(parser.state.cursor, 3);
}

// endregion

// region url

// can not be json-encoded
// can not be empty
// TODO: refacto with hurl_vaue_text?
// but more restrictive: whitelist characters, not empty
pub fn url(p: &mut Parser) -> ParseResult<'static, HurlTemplate> {
    let start = p.clone().state;
    let mut elements = vec![];
    let mut buffer = String::from("");

    if p.is_eof() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Url {},
        });
    }

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
                            if c.is_alphanumeric()
                                || vec![':', '/', '.', '-', '?', '=', '&', '_', '%'].contains(&c)
                            {
                                buffer.push(c);
                            } else {
                                p.state = save.clone();
                                break;
                            }
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

    if elements.is_empty() {
        p.state = start.clone();
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Url { },
        });
    }
    return Ok(HurlTemplate {
        elements,
        //encoded: None,
        delimiter: "".to_string(),
        source_info: SourceInfo {
            start: start.pos,
            end: p.state.clone().pos,
        },
    });
}

#[test]
fn test_url() {
    let mut parser = Parser::init("http://google.fr # ");
    assert_eq!(
        url(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("http://google.fr"),
                    encoded: None,
                }
            }],
            delimiter: "".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 17),
        }
    );
    assert_eq!(parser.state.cursor, 16);
}

#[test]
fn test_url_with_expression() {
    let mut parser = Parser::init("http://{{host}}.fr ");
    assert_eq!(
        url(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![
                HurlTemplateElement::Literal {
                    value: HurlString2 {
                        value: String::from("http://"),
                        encoded: None,
                    }
                },
                HurlTemplateElement::Expression {
                    value: Expr {
                        space0: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(1, 10, 1, 10),
                        },
                        variable: Variable {
                            name: String::from("host"),
                            source_info: SourceInfo::init(1, 10, 1, 14),
                        },
                        space1: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(1, 14, 1, 14),
                        },
                    }
                },
                HurlTemplateElement::Literal {
                    value: HurlString2 {
                        value: String::from(".fr"),
                        encoded: None,
                    }
                },
            ],
            //encoded: None,
            delimiter: "".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 19),
        }
    );
    assert_eq!(parser.state.cursor, 18);
}

#[test]
fn test_url_error_variable() {
    let mut parser = Parser::init("http://{{host>}}.fr");
    let error = url(&mut parser).err().unwrap();
    assert_eq!(
        error.pos,
        Pos {
            line: 1,
            column: 14,
        }
    );
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("}}") });
    assert_eq!(error.recoverable, false);
    assert_eq!(parser.state.cursor, 14);
}

#[test]
fn test_url_error_missing_delimiter() {
    let mut parser = Parser::init("http://{{host");
    let error = url(&mut parser).err().unwrap();
    assert_eq!(
        error.pos,
        Pos {
            line: 1,
            column: 14,
        }
    );
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("}}") });
    assert_eq!(error.recoverable, false);
}

#[test]
fn test_url_error_empty() {
    let mut parser = Parser::init(" # eol");
    let error = url(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Url { });
}

// endregion

// region version
pub fn version(p: &mut Parser) -> ParseResult<'static, Version> {
    try_literal("HTTP/", p)?;
    let available_version = vec![
        ("1.0", VersionValue::Version1),
        ("1.1", VersionValue::Version11),
        ("2", VersionValue::Version2),
        ("*", VersionValue::VersionAny),
    ];
    let start = p.state.clone();
    for (s, value) in available_version {
        match try_literal(s, p) {
            Ok(_) => return Ok(Version {
                value,
                source_info: SourceInfo::init(
                    start.pos.line,
                start.pos.column,
                p.state.pos.line,
                p.state.pos.column,
                ),
            }),
            _ => {}
        }
    }
    return Err(Error {
        pos: start.pos,
        recoverable: false,
        inner: ParseError::Version {},
    });
}

#[test]
fn test_version() {
    let mut parser = Parser::init("HTTP/1.1 200");
    assert_eq!(version(&mut parser).unwrap().value, VersionValue::Version11);

    let mut parser = Parser::init("HTTP/1. 200");
    let error = version(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 6 });
}
// endregion

// region status
pub fn status(p: &mut Parser) -> ParseResult<'static, Status> {
    let start = p.state.clone();
    return match natural(p) {
        Ok(value) => Ok(Status {
            value,
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
            inner: ParseError::Status {},
        }),
    };
}

#[test]
fn test_status() {
    let mut parser = Parser::init("200");
    let s = status(&mut parser).unwrap();
    assert_eq!(s.value, 200);

    let mut parser = Parser::init("xxx");
    let result = status(&mut parser);
    assert!(result.is_err());
    // assert!(result.err().unwrap().pos, Pos { line: 1, column: 1 });
}
// endregion

// region header

pub fn header(p: &mut Parser) -> ParseResult<'static, Header> {
    let line_terminators = optional_line_terminators(p)?;
    let space0 = zero_or_more_spaces(p)?;
    let name = header_name(p)?;
    let space1 = zero_or_more_spaces(p)?;
    recover(|p1| literal(":", p1), p)?;
    let space2 = zero_or_more_spaces(p)?;
    let value = header_value(p)?;
    let line_terminator0 = line_terminator(p)?;
    return Ok(Header {
        line_terminators,
        space0,
        name,
        space1,
        space2,
        value,
        line_terminator0,
    });
}

#[test]
fn test_header() {
    let mut parser = Parser::init("Foo:Bar");
    let header = header(&mut parser).unwrap();
    assert_eq!(
        header.name,
        HurlString {
            value: String::from("Foo"),
            encoded: None,
            source_info: SourceInfo::init(1, 1, 1, 4),
        }
    );
    assert_eq!(
        header.space0,
        Whitespace {
            value: "".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 1),
        }
    );
    assert_eq!(
        header.value,
        HurlTemplate {
            elements: vec![HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("Bar"),
                    encoded: None,
                }
            }],
            delimiter: "".to_string(),
            source_info: SourceInfo::init(1, 5, 1, 8),
        }
    );
}

// endregion

// region header-name
pub fn header_name(p: &mut Parser) -> ParseResult<'static, HurlString> {
    //  let start = p.state.clone();
    let n = name(p)?;
    return Ok(n);
}

#[test]
fn test_header_name() {
    let mut parser = Parser::init("Foo");
    assert_eq!(
        header_name(&mut parser).unwrap(),
        HurlString {
            value: String::from("Foo"),
            encoded: None,
            source_info: SourceInfo::init(1, 1, 1, 4),
        }
    );
}

#[test]
fn test_header_name_error() {
    let mut parser = Parser::init("\"Foo");
    let error = header_name(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 5 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("\"") });
    //println!("{:?}", error);
}
// endregion

// region header-value

pub fn header_value(p: &mut Parser) -> ParseResult<'static, HurlTemplate> {
    //    let start = p.state.clone();
    let value = hurl_value(p)?;
    return Ok(value);
}

#[test]
fn test_header_value() {
    let mut parser = Parser::init("Bar");
    assert_eq!(
        header_value(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("Bar"),
                    encoded: None,
                },
            }],
            //encoded: None,
            source_info: SourceInfo::init(1, 1, 1, 4),
            delimiter: "".to_string(),
        }
    );
}

#[test]
fn test_header_value_json_with_error() {
    let mut parser = Parser::init("\"H\\u0065llo {{ name>}}!\"");
    let error = header_value(&mut parser).err().unwrap();
    assert_eq!(
        error.pos,
        Pos {
            line: 1,
            column: 20,
        }
    );
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("}}") });
}

// endregion

// region body

pub fn body(p: &mut Parser) -> ParseResult<'static, Body> {
    //  let start = p.state.clone();
    let line_terminators = optional_line_terminators(p)?;
    let space0 = zero_or_more_spaces(p)?;
    let value = bytes(p)?;
    let line_terminator0 = line_terminator(p)?;
    return Ok(Body {
        line_terminators,
        space0,
        value,
        line_terminator0,
    });
}

#[test]
fn test_body_json() {
    let mut parser = Parser::init("{}");
    let b = body(&mut parser).unwrap();
    assert_eq!(b.line_terminators.len(), 0);
    assert_eq!(
        b.value,
        Bytes::Json {
            value: String::from("{}")
        }
    );
    assert_eq!(parser.state.cursor, 2);

    let mut parser = Parser::init("# comment\n {} # comment\nxxx");
    let b = body(&mut parser).unwrap();
    assert_eq!(b.line_terminators.len(), 1);
    assert_eq!(
        b.value,
        Bytes::Json {
            value: String::from("{}")
        }
    );
    assert_eq!(parser.state.cursor, 24);

    let mut parser = Parser::init("{x");
    let error = body(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 3 });
    assert_eq!(error.recoverable, false);
}

// endregion

//
// Sections
//

// region request-section
pub fn request_sections(p: &mut Parser) -> ParseResult<'static, Vec<Section>> {
    let sections = zero_or_more(|p1| section(p1), p)?;
    return Ok(sections);
}
// endregion

// region response-sections
pub fn response_sections(p: &mut Parser) -> ParseResult<'static, Vec<Section>> {
    let sections = zero_or_more(|p1| section(p1), p)?;
    return Ok(sections);
}
// endregion

// region section

pub fn section(p: &mut Parser) -> ParseResult<'static, Section> {
    let line_terminators = optional_line_terminators(p)?;
    let space0 = zero_or_more_spaces(p)?;
    let start = p.state.clone();
    let name = section_name(p)?;
    let line_terminator0 = line_terminator(p)?;
    let value = match name.as_str() {
        "QueryStringParams" => section_value_query_params(p)?,
        "FormParams" => section_value_form_params(p)?,
        "Cookies" => section_value_cookies(p)?,
        "Captures" => section_value_captures(p)?,
        "Asserts" => section_value_asserts(p)?,
        _ => {
            return Err(Error {
                pos: Pos {
                    line: start.pos.line,
                    column: start.pos.column + 1,
                },
                recoverable: false,
                inner: ParseError::SectionName { name: name.clone() },
            });
        }
    };
    return Ok(Section {
        line_terminators,
        space0,
        line_terminator0,
        value,
    });
}

#[test]
fn test_asserts_section() {
    let mut parser = Parser::init("[Asserts]\nheader Location equals \"https://google.fr\"\n");

    assert_eq!(
        section(&mut parser).unwrap(),
        Section {
            line_terminators: vec![],
            space0: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(1, 1, 1, 1),
            },
            line_terminator0: LineTerminator {
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 10, 1, 10),
                },
                comment: None,
                newline: Whitespace {
                    value: String::from("\n"),
                    source_info: SourceInfo::init(1, 10, 2, 1),
                },
            },
            value: SectionValue::Asserts(vec![Assert {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(2, 1, 2, 1),
                },
                query: Query {
                    source_info: SourceInfo::init(2, 1, 2, 16),
                    value: QueryValue::Header {
                        space0: Whitespace {
                            value: String::from(" "),
                            source_info: SourceInfo::init(2, 7, 2, 8),
                        },
                        name: HurlString {
                            value: String::from("Location"),
                            encoded: None,
                            source_info: SourceInfo::init(2, 8, 2, 16),
                        },
                    },
                },
                space1: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(2, 16, 2, 17),
                },
                predicate: Predicate {
                    not: false,
                    space0: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(2, 17, 2, 17),
                    },
                    predicate_func: PredicateFunc {
                        source_info: SourceInfo::init(2, 17, 2, 43),
                        value: PredicateFuncValue::EqualString {
                            space0: Whitespace {
                                value: String::from(" "),
                                source_info: SourceInfo::init(2, 23, 2, 24),
                            },
                            value: HurlTemplate {
                                elements: vec![HurlTemplateElement::Literal {
                                    value: HurlString2 {
                                        value: String::from("https://google.fr"),
                                        encoded: Some(String::from("https://google.fr")),
                                    }
                                }],
                                delimiter: "\"".to_string(),
                                source_info: SourceInfo::init(2, 24, 2, 43),
                            },
                        },
                    },
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(2, 43, 2, 43),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::from("\n"),
                        source_info: SourceInfo::init(2, 43, 3, 1),
                    },
                },
            }]),
        }
    );
}

#[test]
fn test_asserts_section_error() {
    let mut parser = Parser::init("x[Assertsx]\nheader Location equals \"https://google.fr\"\n");
    let error = section(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, ParseError::Expecting { value: String::from("[") });
    assert_eq!(error.recoverable, true);

    let mut parser = Parser::init("[Assertsx]\nheader Location equals \"https://google.fr\"\n");
    let error = section(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 2 });
    assert_eq!(
        error.inner,
        ParseError::SectionName { name: String::from("Assertsx") }
    );
    assert_eq!(error.recoverable, false);
}
// endregion

// region section-name

pub fn section_name(p: &mut Parser) -> ParseResult<'static, String> {
    try_literal("[", p)?;
    let name = p.next_chars_while(|c| c.is_alphanumeric());
    literal("]", p)?;
    return Ok(name);
}

#[test]
fn test_section_name() {
    let mut parser = Parser::init("[SectionA]");
    assert_eq!(section_name(&mut parser).unwrap(), String::from("SectionA"));
}

// endregion

// region section-value

pub fn section_value_query_params(p: &mut Parser) -> ParseResult<'static, SectionValue> {
    let items = zero_or_more(|p1| param(p1), p)?;
    return Ok(SectionValue::QueryParams(items));
}

pub fn section_value_form_params(p: &mut Parser) -> ParseResult<'static, SectionValue> {
    let items = zero_or_more(|p1| param(p1), p)?;
    return Ok(SectionValue::FormParams(items));
}

pub fn section_value_cookies(p: &mut Parser) -> ParseResult<'static, SectionValue> {
    let items = zero_or_more(|p1| cookie(p1), p)?;
    return Ok(SectionValue::Cookies(items));
}

pub fn section_value_captures(p: &mut Parser) -> ParseResult<'static, SectionValue> {
    let items = zero_or_more(|p1| capture(p1), p)?;
    return Ok(SectionValue::Captures(items));
}

// section-value-asserts
pub fn section_value_asserts(p: &mut Parser) -> ParseResult<'static, SectionValue> {
    let asserts = zero_or_more(|p1| assert(p1), p)?;
    return Ok(SectionValue::Asserts(asserts));
}

// endregion

// region param

pub fn param(p: &mut Parser) -> ParseResult<'static, Param> {
    let line_terminators = optional_line_terminators(p)?;
    let space0 = zero_or_more_spaces(p)?;
    let name = header_name(p)?;
    let space1 = zero_or_more_spaces(p)?;
    recover(|p1| literal(":", p1), p)?;
    let space2 = zero_or_more_spaces(p)?;
    let value = header_value(p)?;
    let line_terminator0 = line_terminator(p)?;
    return Ok(Param {
        line_terminators,
        space0,
        name,
        space1,
        space2,
        value,
        line_terminator0,
    });
}

// endregion

// region param-name

pub fn param_name(p: &mut Parser) -> ParseResult<'static, HurlString> {
    //   let start = p.state.clone();
    let n = name(p)?;
    return Ok(n);
}

#[test]
fn test_param_name() {
    let mut parser = Parser::init("Foo");
    assert_eq!(
        param_name(&mut parser).unwrap(),
        HurlString {
            value: String::from("Foo"),
            encoded: None,
            source_info: SourceInfo::init(1, 1, 1, 4),
        }
    );
}
// endregion

// region param-value

pub fn param_value(p: &mut Parser) -> ParseResult<'static, HurlTemplate> {
    //  let start = p.state.clone();
    let value = hurl_value(p)?;
    return Ok(value);
}

#[test]
fn test_param_value() {
    let mut parser = Parser::init("Bar");
    assert_eq!(
        param_value(&mut parser).unwrap(),
        HurlTemplate {
            elements: vec![HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("Bar"),
                    encoded: None,
                },
            }],
            //encoded: None,
            delimiter: "".to_string(),
            source_info: SourceInfo::init(1, 1, 1, 4),
        }
    );
}

// endregion

// region cookie

pub fn cookie(p: &mut Parser) -> ParseResult<'static, Cookie> {
    // let start = p.state.clone();
    let line_terminators = optional_line_terminators(p)?;
    let space0 = zero_or_more_spaces(p)?;
    let name = cookie_name(p)?;
    let space1 = zero_or_more_spaces(p)?;
    recover(|p1| literal(":", p1), p)?;
    let space2 = zero_or_more_spaces(p)?;
    let value = cookie_value(p)?;
    let line_terminator0 = line_terminator(p)?;
    return Ok(Cookie {
        line_terminators,
        space0,
        name,
        space1,
        space2,
        value,
        line_terminator0,
    });
}

#[test]
fn test_cookie() {
    let mut parser = Parser::init("Foo: Bar");
    let c = cookie(&mut parser).unwrap();
    assert_eq!(c.name.value, String::from("Foo"));
    assert_eq!(c.value.value, String::from("Bar"));
}

// endregion

// region cookie-name
pub fn cookie_name(p: &mut Parser) -> ParseResult<'static, HurlString> {
    //  let start = p.state.clone();
    let n = name(p)?;
    return Ok(n);
}
// endregion

// region cookie-value

pub fn cookie_value(p: &mut Parser) -> ParseResult<'static, CookieValue> {
    //let start = p.state.clone();
    let value = until_line_terminator(p);
    return Ok(CookieValue { value });
}

#[test]
fn test_cookie_value() {
    let mut parser = Parser::init("Bar");
    assert_eq!(
        cookie_value(&mut parser).unwrap(),
        CookieValue {
            value: String::from("Bar")
        }
    );
}

// endregion

// region capture

pub fn capture(p: &mut Parser) -> ParseResult<'static, Capture> {
    let line_terminators = optional_line_terminators(p)?;
    let space0 = zero_or_more_spaces(p)?;
    let name = header_name(p)?;
    let space1 = zero_or_more_spaces(p)?;
    recover(|p1| literal(":", p1), p)?;
    let space2 = zero_or_more_spaces(p)?;
    let q = query(p)?;
    let line_terminator0 = line_terminator(p)?;
    return Ok(Capture {
        line_terminators,
        space0,
        name,
        space1,
        space2,
        query: q,
        line_terminator0,
    });
}

#[test]
fn test_capture() {
    let mut parser = Parser::init("url: header Location");
    let capture0 = capture(&mut parser).unwrap();

    assert_eq!(
        capture0.name,
        HurlString {
            value: String::from("url"),
            encoded: None,
            source_info: SourceInfo::init(1, 1, 1, 4),
        }
    );
    assert_eq!(
        capture0.query,
        Query {
            source_info: SourceInfo::init(1, 6, 1, 21),
            value: QueryValue::Header {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 12, 1, 13),
                },
                name: HurlString {
                    value: String::from("Location"),
                    encoded: None,
                    source_info: SourceInfo::init(1, 13, 1, 21),
                },
            },
        }
    );
}

// endregion

// region assert

pub fn assert(p: &mut Parser) -> ParseResult<'static, Assert> {
    let line_terminators = optional_line_terminators(p)?;
    let space0 = zero_or_more_spaces(p)?;
    let query0 = query(p)?;
    let space1 = one_or_more_spaces(p)?;
    let predicate0 = predicate(p)?;

    // Specifics for jsonpath //
    // jsonpath always return a list
    // the equals predicate will be used as "firstEquals"
    let predicate0 = Predicate {
        not: predicate0.clone().not,
        space0: predicate0.clone().space0,
        predicate_func: PredicateFunc {
            source_info: predicate0.clone().predicate_func.source_info,
            value: if query0.clone().is_jsonpath() {
                match predicate0.clone().predicate_func.value {
                    PredicateFuncValue::EqualBool { space0, value } => PredicateFuncValue::FirstEqualBool { space0, value },
                    PredicateFuncValue::EqualInt { space0, value } => PredicateFuncValue::FirstEqualInt { space0, value },
                    PredicateFuncValue::EqualString { space0, value } => PredicateFuncValue::FirstEqualString { space0, value },
                    PredicateFuncValue::CountEqual { space0, value } => PredicateFuncValue::FirstCountEqual { space0, value },
                    _ => predicate0.clone().predicate_func.value
                }
            } else {
                predicate0.clone().predicate_func.value
            },
        },
    };

    let line_terminator0 = line_terminator(p)?;
    return Ok(Assert {
        line_terminators,
        space0,
        query: query0,
        space1,
        predicate: predicate0,
        line_terminator0,
    });
}

#[test]
fn test_assert() {
    let mut parser = Parser::init("header Location equals \"https://google.fr\"");
    let assert0 = assert(&mut parser).unwrap();

    assert_eq!(
        assert0.query,
        Query {
            source_info: SourceInfo::init(1, 1, 1, 16),
            value: QueryValue::Header {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 7, 1, 8),
                },
                name: HurlString {
                    value: String::from("Location"),
                    encoded: None,
                    source_info: SourceInfo::init(1, 8, 1, 16),
                },
            },
        }
    );
}


#[test]
fn test_assert_jsonpath() {
    let mut parser = Parser::init("jsonpath $.errors equals 5");

    assert_eq!(assert(&mut parser).unwrap().predicate, Predicate {
        not: false,
        space0: Whitespace { value: String::from(""), source_info: SourceInfo::init(1, 19, 1, 19) },
        predicate_func: PredicateFunc {
            source_info: SourceInfo::init(1, 19, 1, 27),
            value: PredicateFuncValue::FirstEqualInt {
                space0: Whitespace { value: String::from(" "), source_info: SourceInfo::init(1, 25, 1, 26) },
                value: 5,
            },
        },
    });
}
// endregion

// region query

pub fn query(p: &mut Parser) -> ParseResult<'static, Query> {
    let start = p.state.pos.clone();
    let value = query_value(p)?;
    let end = p.state.pos.clone();
    return Ok(Query {
        source_info: SourceInfo { start, end },
        value,
    });
}

#[test]
fn test_query() {
    let mut parser = Parser::init("status");
    assert_eq!(query(&mut parser).unwrap(), Query {
        source_info: SourceInfo::init(1, 1, 1, 7),
        value: QueryValue::Status {},
    });
}


// endregion

// region query-value

pub fn query_value(p: &mut Parser) -> ParseResult<'static, QueryValue> {
    return choice(
        vec![
            status_query,
            header_query,
            cookie_query,
            body_query,
            xpath_query,
            jsonpath_query,
        ],
        p,
    );
}

pub fn status_query(p: &mut Parser) -> ParseResult<'static, QueryValue> {
    try_literal("status", p)?;
    return Ok(QueryValue::Status {});
}

#[test]
fn test_status_query() {
    let mut parser = Parser::init("status");
    assert_eq!(query(&mut parser).unwrap(), Query {
        source_info: SourceInfo::init(1, 1, 1, 7),
        value: QueryValue::Status {},
    });
}

pub fn header_query(p: &mut Parser) -> ParseResult<'static, QueryValue> {
    try_literal("header", p)?;
    let space0 = one_or_more_spaces(p)?;
    let name = name(p)?;
    return Ok(QueryValue::Header { space0, name });
}

#[test]
fn test_header_query() {
    let mut parser = Parser::init("header Foo");
    assert_eq!(
        header_query(&mut parser).unwrap(),
        QueryValue::Header {
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 7, 1, 8),
            },
            name: HurlString {
                value: "Foo".to_string(),
                encoded: None,
                source_info: SourceInfo::init(1, 8, 1, 11),
            },
        }
    );
}

pub fn cookie_query(p: &mut Parser) -> ParseResult<'static, QueryValue> {
    try_literal("cookie", p)?;
    let space0 = one_or_more_spaces(p)?;
    let name = name(p)?;
    return Ok(QueryValue::Cookie { space0, name });
}

#[test]
fn test_cookie_query() {
    let mut parser = Parser::init("cookie Foo");
    assert_eq!(
        cookie_query(&mut parser).unwrap(),
        QueryValue::Cookie {
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 7, 1, 8),
            },
            name: HurlString {
                value: "Foo".to_string(),
                encoded: None,
                source_info: SourceInfo::init(1, 8, 1, 11),
            },
        }
    );
}


pub fn body_query(p: &mut Parser) -> ParseResult<'static, QueryValue> {
    try_literal("body", p)?;
    return Ok(QueryValue::Body {});
}

pub fn xpath_query(p: &mut Parser) -> ParseResult<'static, QueryValue> {
    recover(|p1| literal("xpath", p1), p)?;
    let space0 = one_or_more_spaces(p)?;
    let expr = xpath_expr(p)?;
    return Ok(QueryValue::Xpath { space0, expr });
}

#[test]
fn test_xpath_query() {
    let mut parser = Parser::init("xpath normalize-space(//head/title)");
    assert_eq!(
        xpath_query(&mut parser).unwrap(),
        QueryValue::Xpath {
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 6, 1, 7),
            },
            expr: HurlString {
                value: String::from("normalize-space(//head/title)"),
                encoded: None,
                source_info: SourceInfo::init(1, 7, 1, 36),
            },
        },
    );

    let mut parser = Parser::init("xpath \"normalize-space(//div[contains(concat(' ',normalize-space(@class),' '),' monthly-price ')])\"");
    assert_eq!(xpath_query(&mut parser).unwrap(), QueryValue::Xpath {
        space0: Whitespace { value: String::from(" "), source_info: SourceInfo::init(1, 6, 1, 7) },
        expr: HurlString {
            value: String::from("normalize-space(//div[contains(concat(' ',normalize-space(@class),' '),' monthly-price ')])"),
            encoded: Some(String::from("normalize-space(//div[contains(concat(' ',normalize-space(@class),' '),' monthly-price ')])")),
            source_info: SourceInfo::init(1, 7, 1, 100),
        },
    });
}

pub fn jsonpath_query(p: &mut Parser) -> ParseResult<'static, QueryValue> {
    try_literal("jsonpath", p)?;
    let space0 = one_or_more_spaces(p)?;
    let expr = jsonpath_expr(p)?;
    return Ok(QueryValue::Jsonpath { space0, expr });
}

#[test]
fn test_jsonpath_query() {
    let mut parser = Parser::init("jsonpath $['statusCode']");
    assert_eq!(
        jsonpath_query(&mut parser).unwrap(),
        QueryValue::Jsonpath {
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 9, 1, 10),
            },
            expr: HurlString {
                value: String::from("$['statusCode']"),
                encoded: None,
                source_info: SourceInfo::init(1, 10, 1, 25),
            },
        },
    );
    let mut parser = Parser::init("jsonpath $.success");
    assert_eq!(
        jsonpath_query(&mut parser).unwrap(),
        QueryValue::Jsonpath {
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 9, 1, 10),
            },
            expr: HurlString {
                value: String::from("$.success"),
                encoded: None,
                source_info: SourceInfo::init(1, 10, 1, 19),
            },
        },
    );
}

pub fn xpath_expr(p: &mut Parser) -> ParseResult<'static, HurlString> {
    let start = p.state.clone();
    return match json::json_string2(p) {
        Ok((value, encoded)) => {
//let encoded = p.buffer[start.cursor..p.state.cursor].to_string();
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
                let value = p.next_chars_while(|c| {
                    c.is_alphanumeric()
                        || vec![
                        '-', '(', ')', '[', ']', '{', '}', '/', '@', ',', '\'', '_', '=', '"',
                    ]
                        .contains(&c)
                });
                if value == "" {
                    return Err(Error {
                        pos: start.pos,
                        recoverable: false,
                        inner: ParseError::XPathExpr {},
                    });
                }
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

pub fn jsonpath_expr(p: &mut Parser) -> ParseResult<'static, HurlString> {
    let start = p.state.clone();
    match json::json_string(p) {
        Ok(v) => {
            let encoded = String::from("TBD"); //p.deprecated_buffer[start.cursor..p.state.cursor].chars().as_str().to_string();
            return Ok(HurlString {
                value: v,
                encoded: Some(encoded),
                source_info: SourceInfo {
                    start: start.pos,
                    end: p.clone().state.pos,
                },
            });
        }
        Err(e) => {
            if e.recoverable {
                let value = p.next_chars_while(|c| {
                    c.is_alphanumeric()
                        || *c == '-'
                        || *c == '.'
                        || *c == '('
                        || *c == ')'
                        || *c == '/'
                        || *c == '$'
                        || *c == '\\'
                        || *c == '['
                        || *c == ']'
                        || *c == '\''
                });
                if value == "" {
                    return Err(Error {
                        pos: start.pos,
                        recoverable: false,
                        inner: ParseError::JsonpathExpr {},
                    });
                };
                return Ok(HurlString {
                    value: value.clone(),
                    encoded: None,
                    source_info: SourceInfo {
                        start: start.pos,
                        end: p.clone().state.pos,
                    },
                });
            } else {
                return Err(e);
            }
        }
    };
}
// endregion

// region predicate
// specifics for jsonpath => equals return a first equals
pub fn predicate(p: &mut Parser) -> ParseResult<'static, Predicate> {
    let (not, space0) = match try_literal("not", p) {
        Err(_) => (
            false,
            Whitespace {
                value: String::from(""),
                source_info: SourceInfo {
                    start: p.state.clone().pos,
                    end: p.state.clone().pos,
                },
            },
        ),
        Ok(_) => (true, one_or_more_spaces(p)?),
    };
    let func = predicate_func(p)?;
    return Ok(Predicate {
        not,
        space0,
        predicate_func: func,
    });
}

#[test]
fn test_predicate() {
    let mut parser = Parser::init("not equals true");
    assert_eq!(
        predicate(&mut parser).unwrap(),
        Predicate {
            not: true,
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 4, 1, 5),
            },
            predicate_func: PredicateFunc {
                source_info: SourceInfo::init(1, 5, 1, 16),
                value: PredicateFuncValue::EqualBool {
                    space0: Whitespace {
                        value: String::from(" ").to_string(),
                        source_info: SourceInfo::init(1, 11, 1, 12),
                    },
                    value: true,
                },
            },
        }
    );
}

#[test]
fn test_predicate_error() {
    let mut parser = Parser::init("countEquals true");
    let error = predicate(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 13 });
    assert_eq!(error.recoverable, false);
    assert_eq!(error.inner, ParseError::PredicateValue {});

}

// endregion

// region predicate-func

pub fn predicate_func(p: &mut Parser) -> ParseResult<'static, PredicateFunc> {
    let start = p.state.clone().pos;
    let value = predicate_func_value(p)?;
    let end = p.state.clone().pos;
    return Ok(PredicateFunc {
        source_info: SourceInfo { start, end },
        value,
    });
}


#[test]
fn test_predicate_func() {
    let mut parser = Parser::init("tata equals 1");
    let error = predicate_func(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 1 });
    assert_eq!(error.recoverable, false);
    assert_eq!(error.inner, ParseError::Predicate {});
}

// endregion

// region predicate-func-value
// specifics for jsonpath
pub fn predicate_func_value(p: &mut Parser) -> ParseResult<'static, PredicateFuncValue> {
    let start = p.state.clone();
   return match choice(
        vec![
            equal_predicate,
            count_equal_predicate,
            start_with_predicate,
            contain_predicate,
            match_predicate,
            exist_predicate,
        ],
        p,
    ) {
        Err(Error{recoverable: true,..}) =>  Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Predicate
        }),
        x => x,
    };
}

pub fn equal_predicate(p: &mut Parser) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("equals", p)?;
    let space0 = one_or_more_spaces(p)?;
    let start = p.state.clone();
    return match predicate_value(p) {
        Ok(PredicateValue::Bool { value }) => Ok(PredicateFuncValue::EqualBool { space0, value }),
        Ok(PredicateValue::Int { value }) => Ok(PredicateFuncValue::EqualInt { space0, value }),
        Ok(PredicateValue::Float { value }) => Ok(PredicateFuncValue::EqualFloat { space0, value }),
        Ok(PredicateValue::Template { value }) => Ok(PredicateFuncValue::EqualString { space0, value }),
        _ => Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::PredicateValue {},
        }),
    };
}

#[test]
fn test_equal_predicate() {
    let mut parser = Parser::init("equals  true");
    assert_eq!(
        equal_predicate(&mut parser).unwrap(),
        PredicateFuncValue::EqualBool {
            value: true,
            space0: Whitespace {
                value: String::from("  "),
                source_info: SourceInfo::init(1, 7, 1, 9),
            },
        }
    );

    let mut parser = Parser::init("equals 1.1");
    assert_eq!(
        equal_predicate(&mut parser).unwrap(),
        PredicateFuncValue::EqualFloat {
            value: Float { int: 1, decimal: 100000000000000000, decimal_digits: 1 },
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 7, 1, 8),
            },
        }
    );

    let mut parser = Parser::init("equals 2");
    assert_eq!(
        equal_predicate(&mut parser).unwrap(),
        PredicateFuncValue::EqualInt {
            value: 2,
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 7, 1, 8),
            },
        }
    );

    let mut parser = Parser::init("equals \"Bob\"");
    assert_eq!(
        equal_predicate(&mut parser).unwrap(),
        PredicateFuncValue::EqualString {
            value: HurlTemplate {
                elements: vec![HurlTemplateElement::Literal {
                    value: HurlString2 {
                        value: String::from("Bob"),
                        encoded: Some(String::from("Bob")),
                    }
                }],
                source_info: SourceInfo::init(1, 8, 1, 13),
                delimiter: "\"".to_string(),
            },
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 7, 1, 8),
            },
        }
    );
}

pub fn count_equal_predicate(p: &mut Parser) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("countEquals", p)?;
    let space0 = one_or_more_spaces(p)?;
    let save = p.state.clone();
    let value = match natural(p) {
        Err(_) => return Err(Error {
            pos: save.pos,
            recoverable: false,
            inner: ParseError::PredicateValue {},
        }),
        Ok(value) => value
    };
    return Ok(PredicateFuncValue::CountEqual { space0, value });
}

#[test]
fn test_count_equal_predicate() {
    let mut parser = Parser::init("countEquals 2");
    assert_eq!(
        count_equal_predicate(&mut parser).unwrap(),
        PredicateFuncValue::CountEqual {
            value: 2,
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 12, 1, 13),
            },
        }
    );

    let mut parser = Parser::init("countEquals true");
    let error = count_equal_predicate(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 13 });
    assert_eq!(error.recoverable, false);
    assert_eq!(error.inner, ParseError::PredicateValue {});
}

pub fn start_with_predicate(p: &mut Parser) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("startsWith", p)?;
    let space0 = one_or_more_spaces(p)?;
    let save = p.state.clone();
    let value = match hurl_value_json(p) {
        Err(_) => return Err(Error {
            pos: save.pos,
            recoverable: false,
            inner: ParseError::PredicateValue {},
        }),
        Ok(value) => value,
    };
    return Ok(PredicateFuncValue::StartWith { space0, value });
}

#[test]
fn test_start_with_predicate() {
    let mut parser = Parser::init("startsWith 2");
    let error = start_with_predicate(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 12 });
    assert_eq!(error.recoverable, false);
    assert_eq!(error.inner, ParseError::PredicateValue {});
}

pub fn contain_predicate(p: &mut Parser) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("contains", p)?;
    let space0 = one_or_more_spaces(p)?;
    let value = hurl_value_json(p)?;
    return Ok(PredicateFuncValue::Contain { space0, value });
}

pub fn match_predicate(p: &mut Parser) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("matches", p)?;
    let space0 = one_or_more_spaces(p)?;
    let start = p.clone().state.pos;
    let (value, encoded) = json::json_string2(p)?;
    let end = p.clone().state.pos;
    let value = HurlString {
        value,
        encoded: Some(encoded),
        source_info: SourceInfo { start, end },
    };
    return Ok(PredicateFuncValue::Match { space0, value });
}

pub fn exist_predicate(p: &mut Parser) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("exists", p)?;
    return Ok(PredicateFuncValue::Exist{  });
}

// endregion

// region predicate_value
// internal to the parser

#[derive(Clone, Debug, PartialEq, Eq)]
enum PredicateValue {
    Int { value: i64 },
    Float { value: Float },
    Bool { value: bool },
    Template { value: HurlTemplate },
}

fn predicate_value(p: &mut Parser) -> ParseResult<'static, PredicateValue> {
    return choice(
        vec![
            |p1| match boolean(p1) {
                Ok(value) => Ok(PredicateValue::Bool { value }),
                Err(e) => Err(e),
            },
            |p1| match float(p1) {
                Ok(value) => Ok(PredicateValue::Float { value }),
                Err(e) => Err(e),
            },
            |p1| match integer(p1) {
                Ok(value) => Ok(PredicateValue::Int { value }),
                Err(e) => Err(e),
            },
            |p1| match hurl_value_json(p1) {
                Ok(value) => Ok(PredicateValue::Template { value }),
                Err(e) => Err(e),
            },
        ],
        p,
    );
}

#[test]
fn test_predicate_value() {
    let mut parser = Parser::init("true");
    assert_eq!(
        predicate_value(&mut parser).unwrap(),
        PredicateValue::Bool { value: true }
    );

    let mut parser = Parser::init("1");
    assert_eq!(
        predicate_value(&mut parser).unwrap(),
        PredicateValue::Int { value: 1 }
    );

    let mut parser = Parser::init("1.1");
    assert_eq!(
        predicate_value(&mut parser).unwrap(),
        PredicateValue::Float {
            value: Float { int: 1, decimal: 100000000000000000, decimal_digits: 1 }
        }
    );
}

// endregion
