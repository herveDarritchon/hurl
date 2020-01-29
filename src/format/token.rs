#[cfg(test)]
use crate::core::core::SourceInfo;

use super::super::core::ast::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Method(String),
    Version(String),
    Status(String),
    SectionHeader(String),
    QueryType(String),
    PredicateType(String),
    Not(String),
    Keyword(String),

    // Primitives
    Whitespace(String),
    Comment(String),
    Value(String),
    Colon(String),
    Quote(String),
    Boolean(String),
    Number(String),
    String(String),
    CodeDelimiter(String),
    CodeVariable(String),
}

pub trait Tokenizable {
    fn tokenize(&self) -> Vec<Token>;
}

fn add_tokens(tokens1: &mut Vec<Token>, tokens2: Vec<Token>) {
    for token in tokens2 {
        tokens1.push(token);
    }
}

// region hurl-file
impl Tokenizable for HurlFile {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.entries.iter().flat_map(|e| e.tokenize()).collect(),
        );
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        return tokens;
    }
}
// endregion

// region entry
impl Tokenizable for Entry {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(&mut tokens, self.request.tokenize());
        match self.clone().response {
            Some(response) => add_tokens(&mut tokens, response.tokenize()),
            _ => {}
        }
        return tokens;
    }
}
// endregion

// region request
impl Tokenizable for Request {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        tokens.push(Token::Method(self.method.as_str().to_string()));
        add_tokens(&mut tokens, self.space1.tokenize());
        add_tokens(&mut tokens, self.url.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        add_tokens(
            &mut tokens,
            self.headers.iter().flat_map(|e| e.tokenize()).collect(),
        );
        add_tokens(
            &mut tokens,
            self.sections.iter().flat_map(|e| e.tokenize()).collect(),
        );
        match self.clone().body {
            Some(body) => add_tokens(&mut tokens, body.tokenize()),
            None => {}
        }
        return tokens;
    }
}
// endregion

// region response
impl Tokenizable for Response {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.version.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Status(self.status.value.to_string()));
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        add_tokens(
            &mut tokens,
            self.headers.iter().flat_map(|e| e.tokenize()).collect(),
        );
        add_tokens(
            &mut tokens,
            self.sections.iter().flat_map(|e| e.tokenize()).collect(),
        );
        match self.clone().body {
            Some(body) => add_tokens(&mut tokens, body.tokenize()),
            None => {}
        }
        return tokens;
    }
}
// endregion

// region version
impl Tokenizable for Version {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.push(Token::Version(format!(
            "HTTP/{}",
            match self.value {
                VersionValue::Version1 => String::from("1.0"),
                VersionValue::Version11 => String::from("1.1"),
                VersionValue::Version2 => String::from("2"),
                VersionValue::VersionAny => String::from("*"),
            }
        )));
        return tokens;
    }
}
// endregion

// region header
impl Tokenizable for Header {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.name.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        add_tokens(&mut tokens, self.space2.tokenize());
        add_tokens(&mut tokens, self.value.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        return tokens;
    }
}
// endregion

// region body
impl Tokenizable for Body {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.value.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        return tokens;
    }
}
// endregion

// region bytes
impl Tokenizable for Bytes {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self {
            Bytes::Json { value } => tokens.push(Token::String(value.to_string())),
            Bytes::Xml { value } => {
                tokens.push(Token::String(value.to_string()));
            }
//            Bytes::MultilineString { value: _ } => {}
            Bytes::MultilineString { newline0, value } => {
                tokens.push(Token::Keyword(String::from("```")));
                add_tokens(&mut tokens, newline0.tokenize());
                tokens.push(Token::String(value.to_string()));
                tokens.push(Token::Keyword(String::from("```")));
            }
            Bytes::Base64 {
                space0,
                value: _,
                encoded,
                space1,
            } => {
                tokens.push(Token::Keyword(String::from("base64,")));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.push(Token::String(encoded.to_string()));
                add_tokens(&mut tokens, space1.tokenize());
                tokens.push(Token::Keyword(String::from(";")));
            }
            Bytes::File {
                space0,
                filename,
                space1,
            } => {
                tokens.push(Token::Keyword(String::from("file,")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, filename.tokenize());
                //tokens.push(Token::String(filename.to_string()));
                add_tokens(&mut tokens, space1.tokenize());
                tokens.push(Token::Keyword(String::from(";")));
            }
        }
        return tokens;
    }
}
// endregion

// region section

impl Tokenizable for Section {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        tokens.push(Token::SectionHeader(format!("[{}]", self.name())));
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        add_tokens(&mut tokens, self.value.tokenize());
        //        add_tokens(&mut tokens, self.space0.tokenize());
        //        tokens.push(Token::SectionHeader(format!("[{}]", self.name)));
        return tokens;
    }
}
// endregion

// region section-value

impl Tokenizable for SectionValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self {
            SectionValue::Asserts(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
            SectionValue::QueryParams(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
            SectionValue::FormParams(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
            SectionValue::Cookies(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
            SectionValue::Captures(items) => {
                add_tokens(
                    &mut tokens,
                    items.iter().flat_map(|e| e.tokenize()).collect(),
                );
            }
        }
        return tokens;
    }
}
// endregion

//// region section-param
//impl Tokenizable for Section<Param> {
//    fn tokenize(&self) -> Vec<Token> {
//        let mut tokens: Vec<Token> = vec![];
//        add_tokens(&mut tokens, self.line_terminators.iter().flat_map(|e| e.tokenize()).collect());
//        add_tokens(&mut tokens, self.space0.tokenize());
//        tokens.push(Token::SectionHeader(format!("[{}]", self.name)));
//        add_tokens(&mut tokens, self.line_terminator0.tokenize());
//        add_tokens(&mut tokens, self.items.iter().flat_map(|e| e.tokenize()).collect());
//        return tokens;
//    }
//}
//// endregion
//
//// region section-cookie
//impl Tokenizable for Section<Cookie> {
//    fn tokenize(&self) -> Vec<Token> {
//        let mut tokens: Vec<Token> = vec![];
//        add_tokens(&mut tokens, self.line_terminators.iter().flat_map(|e| e.tokenize()).collect());
//        add_tokens(&mut tokens, self.space0.tokenize());
//        tokens.push(Token::SectionHeader(format!("[{}]", self.name)));
//        add_tokens(&mut tokens, self.line_terminator0.tokenize());
//        add_tokens(&mut tokens, self.items.iter().flat_map(|e| e.tokenize()).collect());
//        return tokens;
//    }
//}
//// endregion

//// region section-cpature
//impl Tokenizable for Section<Capture> {
//    fn tokenize(&self) -> Vec<Token> {
//        let mut tokens: Vec<Token> = vec![];
//        add_tokens(&mut tokens, self.line_terminators.iter().flat_map(|e| e.tokenize()).collect());
//        add_tokens(&mut tokens, self.space0.tokenize());
//        tokens.push(Token::SectionHeader(format!("[{}]", self.name)));
//        add_tokens(&mut tokens, self.line_terminator0.tokenize());
//        add_tokens(&mut tokens, self.items.iter().flat_map(|e| e.tokenize()).collect());
//        return tokens;
//    }
//}
//// endregion

//// region section-assert
//impl Tokenizable for Section<Assert> {
//    fn tokenize(&self) -> Vec<Token> {
//        let mut tokens: Vec<Token> = vec![];
//        add_tokens(&mut tokens, self.line_terminators.iter().flat_map(|e| e.tokenize()).collect());
//        add_tokens(&mut tokens, self.space0.tokenize());
//        tokens.push(Token::SectionHeader(format!("[{}]", self.name)));
//        add_tokens(&mut tokens, self.line_terminator0.tokenize());
//        add_tokens(&mut tokens, self.items.iter().flat_map(|e| e.tokenize()).collect());
//        return tokens;
//    }
//}
//// endregion

// region param
impl Tokenizable for Param {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.name.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        add_tokens(&mut tokens, self.space2.tokenize());
        add_tokens(&mut tokens, self.value.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        return tokens;
    }
}
// endregion

// region cookie
impl Tokenizable for Cookie {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.name.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        add_tokens(&mut tokens, self.space2.tokenize());
        add_tokens(&mut tokens, self.value.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        return tokens;
    }
}
// endregion

// region cookie-value
impl Tokenizable for CookieValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.push(Token::Value(self.clone().value));
        return tokens;
    }
}
// endregion

// region capture
impl Tokenizable for Capture {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.name.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::Colon(String::from(":")));
        add_tokens(&mut tokens, self.space2.tokenize());
        add_tokens(&mut tokens, self.query.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        return tokens;
    }
}
// endregion

// region assert
impl Tokenizable for Assert {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(
            &mut tokens,
            self.line_terminators
                .iter()
                .flat_map(|e| e.tokenize())
                .collect(),
        );
        add_tokens(&mut tokens, self.space0.tokenize());
        add_tokens(&mut tokens, self.query.tokenize());
        add_tokens(&mut tokens, self.space1.tokenize());
        add_tokens(&mut tokens, self.predicate.tokenize());
        add_tokens(&mut tokens, self.line_terminator0.tokenize());
        return tokens;
    }
}
// endregion

// region query
impl Tokenizable for Query {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self.value.clone() {
            QueryValue::Status {} => tokens.push(Token::QueryType(String::from("status"))),
            QueryValue::Header { space0, name } => {
                tokens.push(Token::QueryType(String::from("header")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, name.tokenize());
            }
            QueryValue::Cookie { space0, name } => {
                tokens.push(Token::QueryType(String::from("cookie")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, name.tokenize());
            }
            QueryValue::Body {} => tokens.push(Token::QueryType(String::from("body"))),
            QueryValue::Xpath { space0, expr } => {
                tokens.push(Token::QueryType(String::from("xpath")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, expr.tokenize());
            }
            QueryValue::Jsonpath { space0, expr } => {
                tokens.push(Token::QueryType(String::from("jsonpath")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, expr.tokenize());
            }
            QueryValue::Regex { space0, expr } => {
                tokens.push(Token::QueryType(String::from("regex")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, expr.tokenize());
            }
        }
        return tokens;
    }
}
// endregion

// region predicate
impl Tokenizable for Predicate {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        if self.not {
            tokens.push(Token::Not(String::from("not")));
            add_tokens(&mut tokens, self.space0.tokenize());
        }
        add_tokens(&mut tokens, self.predicate_func.tokenize());
        return tokens;
    }
}
// endregion

// region predicate-func
impl Tokenizable for PredicateFunc {
    fn tokenize(&self) -> Vec<Token> {
        return self.value.tokenize();
    }
}
// endregion

// region predicate-func-value
impl Tokenizable for PredicateFuncValue {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self {
            PredicateFuncValue::EqualBool { space0, value } => {
                tokens.push(Token::PredicateType(String::from("equals")));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.push(Token::Boolean(value.to_string()));
            }
            PredicateFuncValue::EqualString { space0, value } => {
                tokens.push(Token::PredicateType(String::from("equals")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }
//            PredicateFuncValue::EqualString { space0, value } => {
//                tokens.push(Token::PredicateType(String::from("equals")));
//                add_tokens(&mut tokens, space0.tokenize());
//                tokens.push(Token::String(value.to_string()));
//            }
            PredicateFuncValue::EqualInt { space0, value } => {
                tokens.push(Token::PredicateType(String::from("equals")));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.push(Token::Number(value.to_string()));
            }
            PredicateFuncValue::EqualFloat { space0, value } => {
                tokens.push(Token::PredicateType(String::from("equals")));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.push(Token::Number(format!("{}", value.to_string())));
            }
            PredicateFuncValue::CountEqual { space0, value } => {
                tokens.push(Token::PredicateType(String::from("countEquals")));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.push(Token::Boolean(value.to_string()));
            }
            PredicateFuncValue::StartWith { space0, value } => {
                tokens.push(Token::PredicateType(String::from("startsWith")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }
            PredicateFuncValue::Contain { space0, value } => {
                tokens.push(Token::PredicateType(String::from("contains")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }
            PredicateFuncValue::Match { space0, value } => {
                tokens.push(Token::PredicateType(String::from("matches")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }
            // used by jsonpath only
            PredicateFuncValue::FirstEqualBool{ space0, value } => {
                tokens.push(Token::PredicateType(String::from("equals")));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.push(Token::Boolean(value.to_string()));
            }
            PredicateFuncValue::FirstEqualInt { space0, value } => {
                tokens.push(Token::PredicateType(String::from("equals")));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.push(Token::Number(value.to_string()));
            }
            PredicateFuncValue::FirstEqualString { space0, value } => {
                tokens.push(Token::PredicateType(String::from("equals")));
                add_tokens(&mut tokens, space0.tokenize());
                add_tokens(&mut tokens, value.tokenize());
            }
            PredicateFuncValue::FirstCountEqual { space0, value } => {
                tokens.push(Token::PredicateType(String::from("countEquals")));
                add_tokens(&mut tokens, space0.tokenize());
                tokens.push(Token::Number(value.to_string()));
            }
            PredicateFuncValue::Exist{} => {
                tokens.push(Token::PredicateType(String::from("exists")));
            }
        }
        return tokens;
    }
}
// endregion

// region hurl-string
impl Tokenizable for HurlString {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self.clone().encoded {
            None => tokens.push(Token::Value(self.clone().value)),
            Some(encoded) => {
                tokens.push(Token::Quote(String::from("\"")));
                tokens.push(Token::Value(encoded));
                tokens.push(Token::Quote(String::from("\"")));
            }
        }
        return tokens;
    }
}
impl Tokenizable for HurlString2 {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        match self.clone().encoded {
            None => {
                tokens.push(Token::Value(self.clone().value));
            }
            Some(encoded) => {
                tokens.push(Token::Quote(String::from("\"")));
                tokens.push(Token::Value(encoded));
                tokens.push(Token::Quote(String::from("\"")));
            }
        }
        return tokens;
    }
}
// endregion

// region hurl-template
impl Tokenizable for HurlTemplate {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        if self.delimiter != "" {
            tokens.push(Token::Quote(self.clone().delimiter));
        }
        for element in self.elements.clone() {
            add_tokens(&mut tokens, element.tokenize());
        }

        if self.delimiter != "" {
            tokens.push(Token::Quote(self.clone().delimiter));
        }
        return tokens;
    }
}

#[test]
fn test_hurl_template() {
    // "H\u0065llo {{ name}}!"
    let template = HurlTemplate {
        elements: vec![
            HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("Hello "),
                    encoded: Some(String::from("H\\u0065llo ")),
                },
            },
            HurlTemplateElement::Expression {
                value: Expr {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                    variable: Variable {
                        name: String::from("name"),
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                    space1: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                },
            },
            HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("!"),
                    encoded: None,
                },
            },
        ],
        delimiter: "\"".to_string(),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    assert_eq!(
        template.tokenize(),
        vec![
            Token::Quote(String::from("\"")),
            Token::String(String::from("H\\u0065llo ")),
            Token::CodeDelimiter(String::from("{{")),
            Token::Whitespace(String::from(" ")),
            Token::CodeVariable(String::from("name")),
            Token::CodeDelimiter(String::from("}}")),
            Token::String(String::from("!")),
            Token::Quote(String::from("\""))
        ]
    );
}

// endregion

// region hurl-template-element
impl Tokenizable for HurlTemplateElement {
    fn tokenize(&self) -> Vec<Token> {
        return match self {
            HurlTemplateElement::Literal {
                value: HurlString2 { value, encoded },
            } => {
                let mut tokens: Vec<Token> = vec![];
                tokens.push(Token::String(match encoded {
                    None => value.to_string(),
                    Some(encoded) => encoded.to_string(),
                }));
                tokens
            }
            HurlTemplateElement::Expression { value } => {
                let mut tokens: Vec<Token> = vec![];
                add_tokens(&mut tokens, value.tokenize());
                tokens
            }
        };
    }
}
// endregion

// region expr
impl Tokenizable for Expr {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.push(Token::CodeDelimiter(String::from("{{")));
        add_tokens(&mut tokens, self.space0.tokenize());
        tokens.push(Token::CodeVariable(self.variable.name.clone()));
        add_tokens(&mut tokens, self.space1.tokenize());
        tokens.push(Token::CodeDelimiter(String::from("}}")));
        return tokens;
    }
}
// endregion

// region line-terminator
impl Tokenizable for LineTerminator {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        add_tokens(&mut tokens, self.space0.tokenize());
        match self.clone().comment {
            Some(comment) => {
                add_tokens(&mut tokens, comment.tokenize());
            }
            _ => {}
        }
        add_tokens(&mut tokens, self.newline.tokenize());
        return tokens;
    }
}
// endregion

// region whitespace
impl Tokenizable for Whitespace {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        if self.value != "" {
            tokens.push(Token::Whitespace(self.clone().value));
        }
        return tokens;
    }
}
// endregion

// region comment
impl Tokenizable for Comment {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.push(Token::Comment(format!("#{}", self.clone().value)));
        return tokens;
    }
}
// endregion

// region filename
impl Tokenizable for Filename {
    fn tokenize(&self) -> Vec<Token> {
        return vec![Token::String(self.clone().value)];
    }
}
// endregion
