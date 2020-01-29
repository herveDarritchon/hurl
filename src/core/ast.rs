use super::core::SourceInfo;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HurlFile {
    pub entries: Vec<Entry>,
    pub line_terminators: Vec<LineTerminator>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub request: Request,
    pub response: Option<Response>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub method: Method,
    pub space1: Whitespace,
    pub url: HurlTemplate,
    pub line_terminator0: LineTerminator,
    pub headers: Vec<Header>,
    pub sections: Vec<Section>,
    //    pub query_params: Option<Section<Param>>,
    //    pub form_params: Option<Section<Param>>,
    //    pub cookies: Option<Section<Cookie>>,
    pub body: Option<Body>,
    pub source_info: SourceInfo,
}

impl Request {
    pub fn querystring_params(self) -> Vec<Param> {
        for section in self.sections {
            match section.value {
                SectionValue::QueryParams(params) => return params,
                _ => {}
            }
        }
        return vec![];
    }
    pub fn form_params(self) -> Vec<Param> {
        for section in self.sections {
            match section.value {
                SectionValue::FormParams(params) => return params,
                _ => {}
            }
        }
        return vec![];
    }
    pub fn cookies(self) -> Vec<Cookie> {
        for section in self.sections {
            match section.value {
                SectionValue::Cookies(cookies) => return cookies,
                _ => {}
            }
        }
        return vec![];
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub line_terminators: Vec<LineTerminator>,
    pub version: Version,
    pub space0: Whitespace,
    pub status: Status,
    pub space1: Whitespace,
    pub line_terminator0: LineTerminator,
    pub headers: Vec<Header>,
    pub sections: Vec<Section>,
    pub body: Option<Body>,
    pub source_info: SourceInfo,
}

impl Response {
    pub fn captures(self) -> Vec<Capture> {
        for section in self.sections {
            match section.value {
                SectionValue::Captures(captures) => return captures,
                _ => {}
            }
        }
        return vec![];
    }
    pub fn asserts(self) -> Vec<Assert> {
        for section in self.sections {
            match section.value {
                SectionValue::Asserts(asserts) => return asserts,
                _ => {}
            }
        }
        return vec![];
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

impl Method {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Method::Get => return "GET",
            Method::Head => return "HEAD",
            Method::Post => return "POST",
            Method::Put => return "PUT",
            Method::Delete => return "DELETE",
            Method::Connect => return "CONNECT",
            Method::Options => return "OPTIONS",
            Method::Trace => return "TRACE",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status {
    pub value: u64,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    pub value: VersionValue,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionValue {
    Version1,
    Version11,
    Version2,
    VersionAny,
}

impl VersionValue {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            VersionValue::Version1 => return "1.0",
            VersionValue::Version11 => return "1.1",
            VersionValue::Version2 => return "2",
            VersionValue::VersionAny => return "*",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub name: HurlString,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub value: HurlTemplate,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub value: Bytes,
    pub line_terminator0: LineTerminator,
}

//
// Sections
//

//#[derive(Clone, Debug, PartialEq, Eq)]
//pub struct Section<T> {
//    pub line_terminators: Vec<LineTerminator>,
//    pub space0: Whitespace,
//    pub name: String,
//    pub line_terminator0: LineTerminator,
//    pub items: Vec<T>,
//}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub line_terminator0: LineTerminator,
    pub value: SectionValue,
}

impl Section {
    pub fn name(&self) -> &str {
        return match self.value {
            SectionValue::Asserts(_) => "Asserts",
            SectionValue::QueryParams(_) => "QueryStringParams",
            SectionValue::FormParams(_) => "FormParams",
            SectionValue::Cookies(_) => "Cookies",
            SectionValue::Captures(_) => "Captures",
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SectionValue {
    QueryParams(Vec<Param>),
    FormParams(Vec<Param>),
    Cookies(Vec<Cookie>),
    Captures(Vec<Capture>),
    Asserts(Vec<Assert>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub name: HurlString,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub value: HurlTemplate,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub name: HurlString,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub value: CookieValue,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CookieValue {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capture {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub name: HurlString,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub query: Query,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assert {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub query: Query,
    pub space1: Whitespace,
    pub predicate: Predicate,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query {
    pub source_info: SourceInfo,
    pub value: QueryValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryValue {
    Status {},
    Header {
        space0: Whitespace,
        name: HurlString,
    },
    Cookie {
        space0: Whitespace,
        name: HurlString,
    },
    Body {},
    Xpath {
        space0: Whitespace,
        expr: HurlString,
    },
    Jsonpath {
        space0: Whitespace,
        expr: HurlString,
    },
    Regex {
        space0: Whitespace,
        expr: HurlString,
    },
}
impl Query {
    pub fn is_jsonpath(self) -> bool {
        return match self.value {
            QueryValue::Jsonpath {..} => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Predicate {
    pub not: bool,
    pub space0: Whitespace,
    pub predicate_func: PredicateFunc,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Not {
    pub value: bool,
    pub space0: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PredicateFunc {
    pub source_info: SourceInfo,
    pub value: PredicateFuncValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PredicateFuncValue {
    EqualString {
        space0: Whitespace,
        value: HurlTemplate,
    },
    EqualInt {
        space0: Whitespace,
        value: i64,
    },
    EqualFloat {
        space0: Whitespace,
        value: Float,
    },
    EqualBool {
        space0: Whitespace,
        value: bool,
    },
    CountEqual {
        space0: Whitespace,
        value: u64,
    },
    StartWith {
        space0: Whitespace,
        value: HurlTemplate,
    },
    Contain {
        space0: Whitespace,
        value: HurlTemplate,
    },
    Match {
        space0: Whitespace,
        value: HurlString,
    },
    FirstEqualInt {
        space0: Whitespace,
        value: i64,
    },
    FirstEqualBool {
        space0: Whitespace,
        value: bool,
    },
    FirstEqualString {
        space0: Whitespace,
        value: HurlTemplate,
    },
    FirstCountEqual {
        space0: Whitespace,
        value: u64,
    },
    Exist {}
}

//
// Primitives
//

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comment {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HurlString {
    pub value: String,
    pub encoded: Option<String>,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HurlString2 {
    pub value: String,
    pub encoded: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Whitespace {
    pub value: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Filename {
    pub value: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Number {
    pub int: i64,
    pub decimal: u64,
}

// keep Number terminology for both Integer and Decimal Numbers
// different representation for the same float value
// 1.01 and 1.010

// TBC: Issue with equality for f64
// represent your float only with integers
// must be easily compared to the core float value
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Float {
    pub int: i64,
    pub decimal: u64,   // use 18 digits
    pub decimal_digits: usize,   // number of digits
}
impl Float {
    pub fn to_string(&self) -> String {
        let decimal_str : String = format!("{:018}",self.decimal).chars().into_iter().take(self.decimal_digits).collect();
        return format!("{}.{}",self.int, decimal_str);
    }
}

#[test]
fn test_float() {
    assert_eq!(Float{ int: 1, decimal: 0, decimal_digits: 1 }.to_string(), "1.0");
    assert_eq!(Float{ int: 1, decimal: 10000000000000000, decimal_digits: 2 }.to_string(), "1.01");
    assert_eq!(Float{ int: 1, decimal: 10000000000000000, decimal_digits: 3 }.to_string(), "1.010");
    assert_eq!(Float{ int: -1, decimal: 333333333333333333, decimal_digits: 3 }.to_string(), "-1.333");
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineTerminator {
    pub space0: Whitespace,
    pub comment: Option<Comment>,
    pub newline: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bytes {
    Json {
        value: String,
    },
    Xml {
        value: String,
    },
//    MultilineString {
//        value: String,
//    },
    MultilineString {
        newline0: Whitespace,
        value: String,
    },
    Base64 {
        space0: Whitespace,
        value: Vec<u8>,
        encoded: String,
        space1: Whitespace,
    },
    File {
        space0: Whitespace,
        filename: Filename,
        space1: Whitespace,
    },
}

//
// template
//
// might include expression
// which can only simple ASCII (even in json value)
// optional delimiter/encoding

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HurlTemplate {
    pub elements: Vec<HurlTemplateElement>,
    pub delimiter: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HurlTemplateElement {
    Literal { value: HurlString2 },
    Expression { value: Expr },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub space0: Whitespace,
    pub variable: Variable,
    pub space1: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub source_info: SourceInfo,
}
