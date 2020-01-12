use super::super::core::ast::*;
use super::color::TerminalColor;
use super::token::*;

#[allow(dead_code)]
pub fn format(hurl_file: HurlFile, color: bool) -> String {
    //eprintln!("{:#?}", hurl_file);
    //eprintln!("{:#?}", hurl_file.tokenize());
    let mut buffer = String::from("");
    for token in hurl_file.tokenize() {
        buffer.push_str(format_token(token, color).as_str());
    }
    return buffer;
}

fn format_token(token: Token, color: bool) -> String {
    return match token {
        Token::Whitespace(value) => value,
        Token::Method(value) => {
            if color {
                TerminalColor::LightYellow.format(value)
            } else {
                value
            }
        }
        Token::Version(value) => value,
        Token::Status(value) => value,
        Token::SectionHeader(value) => {
            if color {
                TerminalColor::Magenta.format(value)
            } else {
                value
            }
        }
        Token::Comment(value) => {
            if color {
                TerminalColor::LightBlack.format(value)
            } else {
                value
            }
        }
        Token::Value(value) => value,
        Token::Colon(value) => value,
        Token::QueryType(value) => {
            if color {
                TerminalColor::LightCyan.format(value)
            } else {
                value
            }
        }
        Token::PredicateType(value) => {
            if color {
                TerminalColor::LightYellow.format(value)
            } else {
                value
            }
        }
        Token::Not(value) => {
            if color {
                TerminalColor::LightYellow.format(value)
            } else {
                value
            }
        }
        Token::Boolean(value) | Token::Number(value) => {
            if color {
                TerminalColor::Cyan.format(value)
            } else {
                value
            }
        }
        Token::String(value) => {
            if color {
                TerminalColor::Green.format(value)
            } else {
                value
            }
        }
        Token::Quote(value) => {
            if color {
                TerminalColor::Green.format(value)
            } else {
                value
            }
        }
        Token::CodeDelimiter(value) => {
            if color {
                TerminalColor::Green.format(value)
            } else {
                value
            }
        }
        Token::CodeVariable(value) => {
            if color {
                TerminalColor::Green.format(value)
            } else {
                value
            }
        }
        Token::Keyword(value) => value,
    };
}
