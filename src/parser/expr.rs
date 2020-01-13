use super::core::*;
use super::error::*;
use super::primitives::*;
use crate::core::ast::*;
use crate::core::core::SourceInfo;

#[cfg(test)]
use crate::core::core::Pos;

// region expr
pub fn parse(p: &mut Parser) -> ParseResult<'static, Expr> {
    // let start = p.state.clone();

    //try_literal("{{", p)?;
    // TODO: TO BE REMOVED ASAP!!
    // try_literal("{{", p)?;
    match try_literal("{{{", p) {
            Err(_) => try_literal("{{", p)?,
            _ => {}
        }

    let space0 = zero_or_more_spaces(p)?;
    let variable = variable_name(p)?;
    let space1 = zero_or_more_spaces(p)?;

    //literal("}}", p)?;
    match try_literal("}}}", p) {
            Err(_) => literal("}}", p)?,
            _ => {}
    }

    return Ok(Expr {
        space0,
        variable,
        space1,
    });
}

#[test]
fn test_expr() {
    let mut parser = Parser::init("{{ name}}");
    assert_eq!(
        parse(&mut parser).unwrap(),
        Expr {
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 3, 1, 4),
            },
            variable: Variable {
                name: String::from("name"),
                source_info: SourceInfo::init(1, 4, 1, 8),
            },
            space1: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(1, 8, 1, 8),
            },
        }
    );

    // TODO: TO BE REMOVED ASAP!!
    let mut parser = Parser::init("{{{ name}}}");
    assert_eq!(
           parse(&mut parser).unwrap(),
           Expr {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 4, 1, 5),
                },
                variable: Variable {
                    name: String::from("name"),
                    source_info: SourceInfo::init(1, 5, 1, 9),
                },
                space1: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 9, 1, 9),
                },
            }
    );

}

#[test]
fn test_expr_error() {
    let mut parser = Parser::init("{{host>}}");
    let error = parse(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 7 });
    assert_eq!(error.inner, ParseError::Expecting {value: String::from("}}")});
    assert_eq!(error.recoverable, false);
}

#[test]
fn test_expr_error_eof() {
    let mut parser = Parser::init("{{host");
    let error = parse(&mut parser).err().unwrap();
    assert_eq!(error.pos, Pos { line: 1, column: 7 });
    assert_eq!(error.inner, ParseError::Expecting {value: String::from("}}")});
    assert_eq!(error.recoverable, false);
}

// endregion

// region variable-name
fn variable_name(p: &mut Parser) -> ParseResult<'static, Variable> {
    let start = p.state.clone();
    let name = p.next_chars_while(|c| c.is_alphanumeric() || *c == '_');
    if name == "" {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::TemplateVariable {},
        });
    }
    return Ok(Variable {
        name: name,
        source_info: SourceInfo::init(
            start.pos.line,
            start.pos.column,
            p.state.pos.line,
            p.state.pos.column,
        ),
    });
}

#[test]
fn test_variable() {
    let mut parser = Parser::init("name");
    assert_eq!(
        variable_name(&mut parser).unwrap(),
        Variable {
            name: String::from("name"),
            source_info: SourceInfo::init(1, 1, 1, 5),
        }
    );
}
// endregion
