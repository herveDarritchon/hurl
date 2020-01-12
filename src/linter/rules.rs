use crate::core::ast::*;
use crate::core::core::SourceInfo;

//#[cfg(test)]
//use crate::core::core::{Pos};

use super::core::{Error, Lintable, LinterError};



// region hurl-file

impl Lintable<HurlFile> for HurlFile {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        for entry in self.entries.clone() {
            errors.append(&mut (entry.errors()));
        }
        return errors;
    }

    fn lint(&self) -> HurlFile {
        return HurlFile {
            entries: self.entries.iter().map(|e| e.lint()).collect(),
            line_terminators: self.line_terminators.clone(),
        };
    }
}

#[test]
fn test_hurl_file() {
    let hurl_file = HurlFile {
        entries: vec![],
        line_terminators: vec![],
    };
    let hurl_file_linted = HurlFile {
        entries: vec![],
        line_terminators: vec![],
    };
    assert_eq!(hurl_file.errors(), vec![]);
    assert_eq!(hurl_file.lint(), hurl_file_linted);
}

// endregion

// region entry

impl Lintable<Entry> for Entry {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        errors.append(&mut (self.request.errors()));
        return errors;
    }

    fn lint(&self) -> Entry {
        return Entry {
            request: self.request.lint(),
            response: match self.clone().response {
                None => None,
                Some(response) => Some(response.lint()),
            },
        };
    }
}

#[test]
fn test_entry() {
    let entry = HurlFile {
        entries: vec![],
        line_terminators: vec![],
    };
    let entry_linted = HurlFile {
        entries: vec![],
        line_terminators: vec![],
    };
    assert_eq!(entry.errors(), vec![]);
    assert_eq!(entry.lint(), entry_linted);
}

// endregion

// region request

impl Lintable<Request> for Request {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        if !self.space0.value.is_empty() {
            errors.push(Error {
                source_info: self.clone().space0.source_info,
                inner: LinterError::UnneccessarySpace {},
            });
        }
        if self.space1.value != " " {
            errors.push(Error {
                source_info: self.clone().space1.source_info,
                inner: LinterError::OneSpace {},
            });
        }
        for error in self.line_terminator0.errors() {
            errors.push(error);
        }
        return errors;
    }

    fn lint(&self) -> Request {
        let line_terminators = self.clone().line_terminators;
        let space0 = empty_whitespace();
        let method = self.clone().method;
        let space1 = one_whitespace();

        let url = self.url.clone();
        let line_terminator0 = self.line_terminator0.lint();
        let headers = self.headers.iter().map(|e| e.lint()).collect();
        let b = match self.clone().body {
            None => None,
            Some(body) => Some(body.lint()),
        };
        let sections = self.sections.clone();
        //        let query_params = match self.clone().query_params {
        //            None => None,
        //            Some(query_params) => Some(query_params.lint())
        //        }.clone();
        //        let form_params = self.form_params.clone();
        //        let cookies = self.cookies.clone();

        let source_info = SourceInfo::init(0, 0, 0, 0);
        return Request {
            line_terminators,
            space0,
            method,
            space1,
            url,
            line_terminator0,
            headers,
            sections,
            body: b,
            source_info,
        };
    }
}

impl Lintable<Response> for Response {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        if !self.space0.value.is_empty() {
            errors.push(Error {
                source_info: self.clone().space0.source_info,
                inner: LinterError::UnneccessarySpace {},
            });
        }
        return errors;
    }

    fn lint(&self) -> Response {
        let line_terminators = self.clone().line_terminators;
        let space0 = empty_whitespace();
        let _version = self.clone().version;
        let space1 = self.clone().space1;
        let _status = self.clone().status;
        let line_terminator0 = self.clone().line_terminator0;
        let headers = self.headers.iter().map(|e| e.lint()).collect();
        let sections = self.sections.iter().map(|e| e.lint()).collect();
        let b = self.body.clone();
        return Response {
            line_terminators,
            space0,
            version: _version,
            space1,
            status: _status,
            line_terminator0,
            headers,
            sections,
            body: b,
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
    }
}

impl Lintable<Section> for Section {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        return errors;
    }

    fn lint(&self) -> Section {
        let line_terminators = self.clone().line_terminators;
        return Section {
            line_terminators,
            space0: self.clone().space0,
            value: self.value.lint(),
            line_terminator0: self.clone().line_terminator0,
        };
    }
}

impl Lintable<SectionValue> for SectionValue {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        return errors;
    }

    fn lint(&self) -> SectionValue {
        return match self {
            SectionValue::QueryParams(params) => {
                SectionValue::QueryParams(params.iter().map(|e| e.lint()).collect())
            }
            SectionValue::Captures(captures) => {
                SectionValue::Captures(captures.iter().map(|e| e.lint()).collect())
            }
            SectionValue::Asserts(asserts) => {
                SectionValue::Asserts(asserts.iter().map(|e| e.lint()).collect())
            }
            _ => SectionValue::QueryParams(vec![]),
        };
    }
}

impl Lintable<Param> for Param {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        return errors;
    }

    fn lint(&self) -> Param {
        return Param {
            line_terminators: self.clone().line_terminators,
            space0: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            name: self.clone().name,
            space1: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            space2: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            value: self.clone().value,
            line_terminator0: self.clone().line_terminator0,
        };
    }
}

impl Lintable<Assert> for Assert {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        return errors;
    }

    fn lint(&self) -> Assert {
        return self.clone();
    }
}

impl Lintable<Capture> for Capture {
    fn errors(&self) -> Vec<Error> {
        let errors = vec![];
        return errors;
    }

    fn lint(&self) -> Capture {
        return self.clone();
    }
}

fn empty_whitespace() -> Whitespace {
    return Whitespace {
        value: "".to_string(),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
}

fn one_whitespace() -> Whitespace {
    return Whitespace {
        value: " ".to_string(),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
}

//#[test]
//fn test_request() {
//    let request = Request {
//        line_terminators: vec![],
//        space0: Whitespace {
//            value: " ".to_string(),
//            source_info: SourceInfo::init(1, 1, 1, 2),
//        },
//        method: Method::Get,
//        space1: Whitespace {
//            value: " ".to_string(),
//            source_info: SourceInfo::init(1, 4, 1, 1),
//        },
//        url: HurlTemplate {
//            elements: vec![],
//            delimiter: "".to_string(),
//            source_info: SourceInfo::init(0, 0, 0, 0),
//        }, // HurlTemplate::literal("http://google.fr".to_string()),
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
//        query_params: None,
//        form_params: None,
//        cookies: None,
//        body: None,
//        source_info: SourceInfo::init(1, 1, 1, 21),
//    };
//    let request_linted = Request {
//        line_terminators: vec![],
//        space0: Whitespace {
//            value: "".to_string(),
//            source_info: SourceInfo::init(0, 0, 0, 0),
//        },
//        method: Method::Get,
//        space1: Whitespace {
//            value: " ".to_string(),
//            source_info: SourceInfo::init(0, 0, 0, 0),
//        },
//        url: HurlTemplate {
//            elements: vec![],
//            delimiter: "".to_string(),
//            source_info: SourceInfo::init(0, 0, 0, 0),
//        }, //HurlTemplate::literal("http://google.fr".to_string()),
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
//        query_params: None,
//        form_params: None,
//        cookies: None,
//        body: None,
//        source_info: SourceInfo::init(0, 0, 0, 0),
//    };
//    assert_eq!(
//        request.errors(),
//        vec![Error {
//            source_info: SourceInfo {
//                start: Pos { line: 1, column: 1 },
//                end: Pos { line: 1, column: 2 },
//            },
//            inner: LinterError::UnneccessarySpace {},
//        }]
//    );
//    assert_eq!(request.lint(), request_linted);
//}
// endregion

//     let alphanum_characters : Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const ALPHANUM_CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[allow(dead_code)]
const TEMPLATE_CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

// region header
impl Lintable<Header> for Header {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        if !self.space0.value.is_empty() {
            errors.push(Error {
                source_info: self.clone().space0.source_info,
                inner: LinterError::UnneccessarySpace {},
            });
        }
        match error_hurl_string(self.clone().name, ALPHANUM_CHARACTERS.chars().collect()) {
            Some(e) => errors.push(e),
            _ => {}
        }
        if !self.space1.value.is_empty() {
            errors.push(Error {
                source_info: self.clone().space0.source_info,
                inner: LinterError::UnneccessarySpace {},
            });
        }
        if self.space2.value != " " {
            errors.push(Error {
                source_info: self.clone().space2.source_info,
                inner: LinterError::OneSpace {},
            });
        }

        for e in error_hurl_template(self.clone().value, ALPHANUM_CHARACTERS.chars().collect()) {
            errors.push(e);
        }

        return errors;
    }

    fn lint(&self) -> Header {
        let empty_space = Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let one_space = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let line_terminators = self.clone().line_terminators;
        let space0 = empty_space.clone();
        let name = lint_hurl_string(self.clone().name, ALPHANUM_CHARACTERS.chars().collect());
        let space1 = empty_space.clone();
        let space2 = one_space.clone();
        let value = lint_hurl_template(self.clone().value, ALPHANUM_CHARACTERS.chars().collect());
        let line_terminator0 = self.clone().line_terminator0;
        return Header {
            line_terminators,
            space0,
            name,
            space1,
            space2,
            value,
            line_terminator0,
        };
    }
}

#[test]
fn test_header() {
    // "Foo":"Bar"
    //Foo: Bar
    let header = Header {
        line_terminators: vec![],
        space0: Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(1, 1, 1, 2),
        },
        name: HurlString {
            value: String::from("Foo"),
            encoded: Some(String::from("Foo")),
            source_info: SourceInfo::init(1, 2, 1, 7),
        },
        space1: Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(1, 0, 0, 0),
        },
        space2: Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(1, 8, 1, 8),
        },
        value: HurlTemplate {
            elements: vec![HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("Bar"),
                    encoded: None,
                },
            }],
            //encoded: Some(String::from("\"Bar\"")),
            delimiter: "".to_string(),
            source_info: SourceInfo::init(1, 8, 1, 13),
        },
        line_terminator0: LineTerminator {
            space0: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            comment: None,
            newline: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
        },
    };
    let linted_header = Header {
        line_terminators: vec![],
        space0: Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        name: HurlString {
            value: String::from("Foo"),
            encoded: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        space1: Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        space2: Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        value: HurlTemplate {
            elements: vec![HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("Bar"),
                    encoded: None,
                },
            }],
            //encoded: None,
            delimiter: "".to_string(),
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        line_terminator0: LineTerminator {
            space0: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            comment: None,
            newline: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
        },
    };

    // "Foo":"Bar"
    //Foo: Bar
    assert_eq!(
        header.errors(),
        vec![
            Error {
                source_info: SourceInfo::init(1, 1, 1, 2),
                inner: LinterError::UnneccessarySpace {},
            },
            Error {
                source_info: SourceInfo::init(1, 2, 1, 7),
                inner: LinterError::UnneccessaryJsonEncoding {},
            },
            Error {
                source_info: SourceInfo::init(1, 8, 1, 8),
                inner: LinterError::OneSpace {},
            },
            //Error { source_info: SourceInfo::init(1, 8, 1, 13), inner: LinterError::UnneccessaryJsonEncoding {} },
        ]
    );
    assert_eq!(linted_header.errors(), vec![]);
    assert_eq!(header.lint(), linted_header);
}

// endregion

// region hurl-string
fn lint_hurl_string(s: HurlString, characters: Vec<char>) -> HurlString {
    match s {
        HurlString {
            value: _,
            encoded: None,
            ..
        } => return s,
        HurlString {
            value,
            encoded: Some(encoded),
            ..
        } => {
            for c in value.chars() {
                if !characters.contains(&c) {
                    return HurlString {
                        value,
                        encoded: Some(encoded),
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    };
                }
            }
            return HurlString {
                value,
                encoded: None,
                source_info: SourceInfo::init(0, 0, 0, 0),
            };
        }
    }
}

#[test]
fn test_hurl_string() {
    let characters = vec!['a', 'b', 'c'];
    assert_eq!(
        lint_hurl_string(
            HurlString {
                value: String::from("aaa"),
                encoded: None,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            characters.clone(),
        ),
        HurlString {
            value: String::from("aaa"),
            encoded: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    );
    assert_eq!(
        lint_hurl_string(
            HurlString {
                value: String::from("aaa"),
                encoded: Some(String::from("aaa")),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            characters.clone(),
        ),
        HurlString {
            value: String::from("aaa"),
            encoded: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    );
    assert_eq!(
        lint_hurl_string(
            HurlString {
                value: String::from("aaa"),
                encoded: Some(String::from("\\u0061aa")),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            characters.clone(),
        ),
        HurlString {
            value: String::from("aaa"),
            encoded: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    );
    assert_eq!(
        lint_hurl_string(
            HurlString {
                value: String::from("abcd"),
                encoded: Some(String::from("abcd")),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            characters.clone(),
        ),
        HurlString {
            value: String::from("abcd"),
            encoded: Some(String::from("abcd")),
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    );
    //assert_eq!(1, 2);
}

fn error_hurl_string(s: HurlString, characters: Vec<char>) -> Option<Error> {
    match s {
        HurlString {
            value,
            encoded: Some(_encoded),
            source_info,
        } => {
            for c in value.chars() {
                if !characters.contains(&c) {
                    return None;
                }
            }
            return Some(Error {
                source_info,
                inner: LinterError::UnneccessaryJsonEncoding {},
            });
        }
        _ => {}
    }
    return None;
}

#[test]
fn test_error_hurl_string() {
    let characters = vec!['a', 'b', 'c'];
    assert_eq!(
        error_hurl_string(
            HurlString {
                value: String::from("aaa"),
                encoded: None,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            characters.clone(),
        ),
        None
    );
    assert_eq!(
        error_hurl_string(
            HurlString {
                value: String::from("abcd"),
                encoded: Some(String::from("abcd")),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            characters.clone(),
        ),
        None
    );

    assert_eq!(
        error_hurl_string(
            HurlString {
                value: String::from("aaa"),
                encoded: Some(String::from("aaa")),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            characters.clone(),
        ),
        Some(Error {
            source_info: SourceInfo::init(0, 0, 0, 0),
            inner: LinterError::UnneccessaryJsonEncoding {},
        })
    );
    assert_eq!(
        error_hurl_string(
            HurlString {
                value: String::from("aaa"),
                encoded: Some(String::from("\\u0061aa")),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            characters.clone(),
        ),
        Some(Error {
            source_info: SourceInfo::init(0, 0, 0, 0),
            inner: LinterError::UnneccessaryJsonEncoding {},
        })
    );
}
// endregion

// region hurl-template

// region hurl-string
fn lint_hurl_template(_templ: HurlTemplate, _characters: Vec<char>) -> HurlTemplate {
    let mut elements = vec![];
    for element in _templ.clone().elements {
        elements.push(lint_hurl_template_element(element.clone()));
    }
    return HurlTemplate {
        elements,
        //encoded: if require_json(templ.clone(), characters) { templ.clone().encoded } else { None },
        delimiter: "".to_string(),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
}

fn lint_hurl_template_element(element: HurlTemplateElement) -> HurlTemplateElement {
    return element.clone();
}

fn require_json(_templ: HurlTemplate, _characters: Vec<char>) -> bool {
    //    for c in templ.value().chars() {
    //        if !characters.contains(&c) {
    //            return true;
    //        }
    //    }
    //return false;
    return true;
}

fn error_hurl_template(_templ: HurlTemplate, characters: Vec<char>) -> Vec<Error> {
    let mut errors = vec![];
    match _templ.clone() {
        HurlTemplate { elements: _, .. } => {
            if !require_json(_templ.clone(), characters) {
                errors.push(Error {
                    source_info: _templ.source_info,
                    inner: LinterError::UnneccessaryJsonEncoding {},
                });
            }
        }
    }
    return errors;
}

#[test]
fn test_hurl_template() {
    // "Hello {{ n\u0061me}}"
    // Hello {{name}}     => remove extra space + json encoding
    // Errors

    // test hurl string => for json encoding
    // test template => no json involved
    // test hurl-template => combine the 2

    let _errors = vec![
        Error {
            source_info: SourceInfo::init(1, 1, 1, 22),
            inner: LinterError::UnneccessaryJsonEncoding {},
        },
        Error {
            source_info: SourceInfo::init(1, 10, 1, 11),
            inner: LinterError::UnneccessarySpace {},
        },
    ];
    let _hurl_template = HurlTemplate {
        elements: vec![
            HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("Hello "),
                    encoded: None,
                },
            },
            HurlTemplateElement::Expression {
                value: Expr {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::init(1, 10, 1, 11),
                    },
                    variable: Variable {
                        name: "name".to_string(),
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                    space1: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                },
            },
        ],
        //encoded: None,
        delimiter: "".to_string(),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };

    //ALPHANUM_CHARACTERS.chars().collect()
}
// endregion

// region template
// not related to json
#[allow(dead_code)]
fn lint_template(templ: HurlTemplate, _characters: Vec<char>) -> HurlTemplate {
    return templ.clone();
}

#[allow(dead_code)]
fn error_template(_templ: HurlTemplate, _characters: Vec<char>) -> Vec<Error> {
    return vec![];
}

#[test]
fn test_template() {
    // Hello {{ name}}
    let _hurl_template = HurlTemplate {
        elements: vec![
            HurlTemplateElement::Literal {
                value: HurlString2 {
                    value: String::from("Hello "),
                    encoded: None,
                },
            },
            HurlTemplateElement::Expression {
                value: Expr {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::init(1, 10, 1, 11),
                    },
                    variable: Variable {
                        name: "name".to_string(),
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                    space1: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                },
            },
        ],
        //encoded: None,
        delimiter: "".to_string(),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
}

// endregion

// region body
impl Lintable<Body> for Body {
    fn errors(&self) -> Vec<Error> {
        unimplemented!()
    }

    fn lint(&self) -> Body {
        let line_terminators = self.clone().line_terminators;
        let space0 = Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let value = self.value.lint();
        let line_terminator0 = self.clone().line_terminator0;
        return Body {
            line_terminators,
            space0,
            value,
            line_terminator0,
        };
    }
}
// endregion

// region body-value
impl Lintable<Bytes> for Bytes {
    fn errors(&self) -> Vec<Error> {
        unimplemented!()
    }

    fn lint(&self) -> Bytes {
        //let space0 = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
        //let value = self.value.lint();
        //let line_terminator0 = self.clone().line_terminator0;
        return match self {
            Bytes::File { filename, .. } => Bytes::File {
                space0: one_whitespace(),
                filename: Filename {
                    value: filename.clone().value,
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                space1: empty_whitespace(),
            },
            Bytes::Base64 { encoded, value, .. } => Bytes::Base64 {
                space0: one_whitespace(),
                value: value.clone(),
                encoded: encoded.clone(),
                space1: empty_whitespace(),
            },
            Bytes::Json { value } => Bytes::Json {
                value: value.clone(),
            },
            Bytes::MultilineString { newline0, value } => Bytes::MultilineString {
                newline0: newline0.clone(),
                value: value.clone(),
            },
            Bytes::Xml { value } => Bytes::Xml {
                value: value.clone(),
            },
//            Bytes::MultilineString { value } => Bytes::MultilineString {
//                value: value.clone(),
//            },
        };
    }
}
// endregion

// region line-ternminator
impl Lintable<LineTerminator> for LineTerminator {
    fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        match self.clone().comment {
            Some(value) => {
                for error in value.errors() {
                    errors.push(error);
                }
            }
            None => {
                if self.space0.value != "" {
                    errors.push(Error {
                        source_info: self.clone().space0.source_info,
                        inner: LinterError::UnneccessarySpace {},
                    });
                }
            }
        }
        return errors;
    }

    fn lint(&self) -> LineTerminator {
        let space0 = match self.comment {
            None => Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            Some(_) => Whitespace {
                value: self.clone().space0.value,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
        };
        let comment = match self.clone().comment {
            None => None,
            Some(comment) => Some(comment.lint()),
        };
        let newline = Whitespace {
            value: if self.newline.value == "" {
                String::from("")
            } else {
                String::from("\n")
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        return LineTerminator {
            space0,
            comment,
            newline,
        };
    }
}
// endregion

// region comment
impl Lintable<Comment> for Comment {
    fn errors(&self) -> Vec<Error> {
        let errors =vec![];

        return errors;
    }

    fn lint(&self) -> Comment {
        return Comment {
            value: if self.value.starts_with(" ") {
                self.clone().value
            } else {
                format!(" {}", self.value).to_string()
            },
        };
    }
}
// endregion
