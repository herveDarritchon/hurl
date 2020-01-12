// template module
// simply pass a full string
// goes to end of line
// combination with json encoding
// tokenization???
// needs json character into the template?
// url does not need that!

use crate::core::ast::*;

use super::core::*;
use super::error::*;
use super::expr;

// region template
#[allow(dead_code)]
pub fn parse(_p: &mut Parser) -> ParseResult<'static, Template> {
    return Ok(Template {
        elements: vec![],
    });
}

#[test]
fn test_template() {
    let mut parser = Parser::init("Hello {{ name}}!");
    assert_eq!(
        parse(&mut parser).unwrap(),
        Template {
            elements: vec![
                TemplateElement::Literal(String::from("Hello ")),
                TemplateElement::Expression(Expr {
                    space0: Whitespace { value: String::from(" "), source_info: SourceInfo::init(1, 14, 1, 20) },
                    variable: Variable { name: String::from("name"), source_info: SourceInfo::init(1, 20, 1, 29) },
                    space1: Whitespace { value: String::from(""), source_info: SourceInfo::init(1, 29, 1, 29) },
                }),
                TemplateElement::Literal(String::from("!")),
            ]
        });
}

// endregion

// region template-element
// not related to json
// works on a decoded stream
#[allow(dead_code)]
fn template_element(p: &mut Parser) -> ParseResult<'static, HurlTemplateElement> {
    let start = p.state.clone();
    if p.clone().is_eof() {
        return Err(Error {
            pos: start.pos,
            recoverable: true,
            inner: ParseError::Template {},
        });
    }

    // This should be code
    match expr::parse(p) {
        Ok(value) => {
            return Ok(HurlTemplateElement::Expression { value });
        }
        Err(e) => {
            if !e.recoverable {
                return Err(e);
            }
        }
    }
    p.state = start;


    // This should be a literal
    let mut s = "".to_string();
    loop {
        let saved_state = p.state.clone();
        match p.next_char() {
            None => {
                break;
            }
            Some(c) => {
                if c != '{' {
                    s.push(c);
                } else {
                    p.state = saved_state.clone();
                    break;
                }
            }
        }
    }
    return Ok(HurlTemplateElement::Literal { value: s });
}

#[test]
fn test_template_element_literal() {
    let mut parser = Parser::init("a{");
    assert_eq!(
        template_element(&mut parser).unwrap(),
        HurlTemplateElement::Literal {
            value: String::from("a")
        }
    );
    assert_eq!(parser.state.cursor, 1);
}

#[test]
fn test_template_element_expression() {
    let mut parser = Parser::init("{{ name}}");
    assert_eq!(
        template_element(&mut parser).unwrap(),
        HurlTemplateElement::Expression {
            value: Expr {
                space0: Whitespace { value: String::from(" "), source_info: SourceInfo::init(1, 3, 1, 4) },
                variable: Variable {
                    name: String::from("name"),
                    source_info: SourceInfo::init(1, 4, 1, 8),
                },
                space1: Whitespace { value: String::from(""), source_info: SourceInfo::init(1, 8, 1, 8) },
            }
        }
    );
}

#[test]
fn test_template_element_expression_error() {
    let mut parser = Parser::init("{{ name_}}");
    let error = template_element(&mut parser).err().unwrap();
    //println!("{:?}", error);
    assert_eq!(error.pos.column, 8);
    assert_eq!(error.inner, ParseError::Unexpected { character: String::from("_") });


    // Error unexpected _ non recoverable
    // Error in TemplateCode + additional message
}


// endregion



