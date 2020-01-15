#[cfg(test)]
use crate::core::core::{Pos, SourceInfo};
use crate::core::core::Value;
use crate::core::jsonpath;
use crate::http;

use super::core::{Error, RunnerError};
//use super::http;
use super::super::core::ast::*;
use super::xpath;

// QueryResult
// success => just the value is kept
// error   => thrown within asserts / logged within Captures

// => hurl report => do you need more information than just the value?
// each assert returned by an entry
// contains a queryLog which contains a query result

// json
// 1.0 and 1 are different?
// 1.01 and 1.010 different on ast => but same on value


// value not found
// Option<Value>

// source info for the input?

// value not found/ does not exist => add error
// depends on the query type
// jsonpath will return an empty list? no match to be clarified
// header return really nothing

// check that header does not exist?
// only for assert   specific not found error to work with predicate exist
// => does not make sense for capture

// implicit assert


// status qnd body never fails!
// semantic not in the type system
// in the test methods availibility


// error source_info for invalid response content
// source info => all the query!! not just the expression! => should come before invalid expression

// error: Invalid XML response
//   --> test.hurl:10:2
// 10 |   xpath //person countEquals 10
//    |   ^^^^^^^^^^^^^^

// error: Invalid XML response
//   --> test.hurl:10:7
// 10 | xpath //person countEquals 10
//    |       ^^^^^^^^

// also use for implicit assert => query header => distinguihed between the 2 header quqry (implicit vs explicit)
// error: Header not Found
//   --> test.hurl:10:2
// 10 |   Custom: XXX
//    |   ^^^^^^
// an enum is always a value?


pub type QueryResult = Result<Value, Error>;


impl Query {
    pub fn eval(self, http_response: http::response::Response) -> QueryResult {
        return match self.value {
            QueryValue::Status {} => Ok(Value::Integer(http_response.status as i64)),
            QueryValue::Header { name: HurlString { value: header_name, .. }, .. } => {
                match http_response.get_header(header_name.as_str(), false) {
                    //None =>  Err(Error { source_info, inner: RunnerError::QueryHeaderNotFound, assert: false }),
                    None => Ok(Value::None),
                    Some(value) => Ok(Value::String(value))
                }
            }
            QueryValue::Cookie { name: HurlString { value: cookie_name, source_info, .. }, .. } => {
                match http_response.get_cookie(cookie_name.as_str()) {
                    None => Err(Error { source_info, inner: RunnerError::QueryCookieNotFound, assert: false }),
                    Some(value) => Ok(Value::String(value))
                }
            }
            QueryValue::Body {} => {
                // can return a string if encoding is known and utf8
                if http_response.has_utf8_body() {
                    match String::from_utf8(http_response.body) {
                        Ok(s) => Ok(Value::String(s)),
                        Err(_) => Err(Error { source_info: self.source_info, inner: RunnerError::QueryInvalidUtf8, assert: false }),
                    }
                } else {
                    Ok(Value::Bytes(http_response.body))
                }
            }
            QueryValue::Xpath { expr: HurlString { value, source_info, .. }, .. } => {
                match String::from_utf8(http_response.clone().body) {
                    Err(_) => Err(Error { source_info: self.source_info.clone(), inner: RunnerError::QueryInvalidUtf8, assert: false }),
                    Ok(xml) => {
                        let result = if http_response.clone().is_html() {
                            xpath::eval_html(xml, value.clone())
                        } else {
                            xpath::eval_xml(xml, value.clone())
                        };
                        match result {
                            Ok(value) =>
                                Ok(value),
                            Err(xpath::XpathError::InvalidXML {}) => Err(Error {
                                source_info: self.source_info,
                                inner: RunnerError::QueryInvalidXml
                                ,
                                assert: false,
                            }),
                            Err(xpath::XpathError::InvalidHtml {}) => Err(Error {
                                source_info: self.source_info,
                                inner: RunnerError::QueryInvalidXml
                                ,
                                assert: false,
                            }),
                            Err(xpath::XpathError::Eval {}) => Err(Error {
                                source_info,
                                inner: RunnerError::QueryInvalidXpathEval,
                                assert: false,
                            }),
                            Err(xpath::XpathError::Unsupported {}) => {
                                panic!("Unsupported xpath {}", value); // good usecase for panic - I could nmot reporduce this usecase myself
                            }
                        }
                    }
                }
            }
            QueryValue::Jsonpath { expr: HurlString { value, source_info, .. }, .. } => {
                let expr = match jsonpath::Expr::init(value.as_str()) {
                    None => return Err(Error { source_info: source_info.clone(), inner: RunnerError::QueryInvalidJsonpathExpression {}, assert: false }),
                    Some(expr) => expr
                };
                let json = match String::from_utf8(http_response.body) {
                    Err(_) => return Err(Error { source_info: self.source_info, inner: RunnerError::QueryInvalidUtf8, assert: false }),
                    Ok(v) => v
                };
                let value = match expr.eval(json.as_str()) {
                    Err(_) => {
                        return Err(Error { source_info: self.source_info, inner: RunnerError::QueryInvalidJson, assert: false });
                    }
                    Ok(value) => {
                        if value == Value::List(vec![]) { Value::None } else { value }
                    }
                };
                return Ok(value);
            }
        };
    }
}


#[cfg(test)]
pub fn xpath_invalid_query() -> Query {
    // xpath ???
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    Query {
        source_info: SourceInfo::init(1, 1, 1, 13),
        value: QueryValue::Xpath {
            space0: whitespace.clone(),
            expr: HurlString {
                value: String::from("???"),
                encoded: None,
                source_info: SourceInfo::init(1, 7, 1, 10),
            },
        },
    }
}


#[cfg(test)]
pub fn xpath_count_user_query() -> Query {
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    Query {
        source_info: SourceInfo::init(1, 1, 1, 13),
        value: QueryValue::Xpath {
            space0: whitespace.clone(),
            expr: HurlString {
                value: String::from("count(//user)"),
                encoded: None,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
        },
    }
}

#[cfg(test)]
pub fn xpath_users() -> Query {
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    Query {
        source_info: SourceInfo::init(1, 1, 1, 13),
        value: QueryValue::Xpath {
            space0: whitespace.clone(),
            expr: HurlString {
                value: String::from("//user"),
                encoded: None,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
        },
    }
}


// region test status

#[test]
fn test_query_status() {
    assert_eq!(
        Query { source_info: SourceInfo::init(0, 0, 0, 0), value: QueryValue::Status {} }.eval(http::response::hello_http_response()).unwrap(),
        Value::Integer(200)
    );
}

// endregion

// region test header

#[test]
fn test_header_not_found() {
    // header Custom
    let query_header = Query {
        source_info: SourceInfo::init(0, 0, 0, 0),
        value: QueryValue::Header {
            space0: Whitespace { value: String::from(" "), source_info: SourceInfo::init(1, 7, 1, 8) },
            name: HurlString {
                value: String::from("Custom"),
                encoded: None,
                source_info: SourceInfo::init(1, 8, 1, 14),
            },
        },
    };
//    let error = query_header.eval(http::hello_http_response()).err().unwrap();
//    assert_eq!(error.source_info.start, Pos { line: 1, column: 8 });
//    assert_eq!(error.inner, RunnerError::QueryHeaderNotFound);
    assert_eq!(query_header.eval(http::response::hello_http_response()).unwrap(), Value::None);
}

#[test]
fn test_header() {
// header Content-Type
    let query_header = Query {
        source_info: SourceInfo::init(0, 0, 0, 0),
        value: QueryValue::Header {
            space0: Whitespace { value: String::from(" "), source_info: SourceInfo::init(1, 7, 1, 8) },
            name: HurlString {
                value: String::from("Content-Type"),
                encoded: None,
                source_info: SourceInfo::init(1, 8, 1, 20),
            },
        },
    };
    assert_eq!(
        query_header.eval(http::response::hello_http_response()).unwrap(),
        Value::String(String::from("text/html; charset=utf-8"))
    );
}

// endregion

// region test body

#[test]
fn test_body() {
    assert_eq!(
        Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: QueryValue::Body {},
        }.eval(http::response::hello_http_response()).unwrap(),
        Value::String(String::from("Hello World!"))
    );
}

// endregion

// region test xpath


#[test]
fn test_query_invalid_utf8() {
    let http_response = http::response::Response {
        version: http::response::Version::Http10,
        status: 0,
        headers: vec![],
        body: vec![200],
    };
    let error = xpath_users().eval(http_response).err().unwrap();
    assert_eq!(error.source_info.start, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, RunnerError::QueryInvalidUtf8);
}

#[test]
fn test_query_xpath_error_eval() {

// xpath ^^^
    let query = Query {
        source_info: SourceInfo::init(0, 0, 0, 0),
        value: QueryValue::Xpath {
            space0: Whitespace {
                value: String::from(" "),
                source_info: SourceInfo::init(1, 6, 1, 7),
            },
            expr: HurlString {
                value: String::from("^^^"),
                encoded: None,
                source_info: SourceInfo::init(1, 7, 1, 10),
            },
        },
    };
    let error = query.eval(http::response::xml_two_users_http_response()).err().unwrap();
    assert_eq!(error.inner, RunnerError::QueryInvalidXpathEval);
    assert_eq!(error.source_info.start, Pos { line: 1, column: 7 });
}

#[test]
fn test_query_xpath() {
    assert_eq!(xpath_users().eval(http::response::xml_two_users_http_response()).unwrap(), Value::Nodeset(2));
    assert_eq!(xpath_count_user_query().eval(http::response::xml_two_users_http_response()).unwrap(), Value::Float(2, 0));
}


#[cfg(test)]
pub fn xpath_html_charset() -> Query {
    // $x("normalize-space(/html/head/meta/@charset)")
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    Query {
        source_info: SourceInfo::init(1, 1, 1, 13),
        value: QueryValue::Xpath {
            space0: whitespace.clone(),
            expr: HurlString {
                value: String::from("normalize-space(/html/head/meta/@charset)"),
                encoded: None,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
        },
    }
}


#[test]
fn test_query_xpath_with_html() {
    assert_eq!(xpath_html_charset().eval(http::response::html_http_response()).unwrap(), Value::String(String::from("UTF-8")));
}


// endregion

// region test jsonpath

#[cfg(test)]
pub fn json_http_response() -> http::response::Response {
    return http::response::Response {
        version: http::response::Version::Http10,
        status: 0,
        headers: vec![],
        body: String::into_bytes(r#"
{
  "success":false,
  "errors": [
    { "id": "error1"},
    {"id": "error2"}
  ]
}
"#.to_string()),
    };
}

#[cfg(test)]
pub fn jsonpath_success() -> Query {
// jsonpath $.success
    return Query {
        source_info: SourceInfo::init(1, 1, 1, 19),
        value: QueryValue::Jsonpath {
            space0: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(1, 9, 1, 10),
            },
            expr: HurlString {
                value: String::from("$.success"),
                encoded: None,
                source_info: SourceInfo::init(1, 10, 1, 19),
            },
        },
    };
}


#[cfg(test)]
pub fn jsonpath_errors() -> Query {
// jsonpath $.errors
    return Query {
        source_info: SourceInfo::init(1, 1, 1, 19),
        value: QueryValue::Jsonpath {
            space0: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(1, 9, 1, 10),
            },
            expr: HurlString {
                value: String::from("$.errors"),
                encoded: None,
                source_info: SourceInfo::init(1, 10, 1, 18),
            },
        },
    };
}


#[test]
fn test_query_jsonpath_invalid_expression() {

// jsonpath xxx
    let jsonpath_query = Query {
        source_info: SourceInfo::init(0, 0, 0, 0),
        value: QueryValue::Jsonpath {
            space0: Whitespace {
                value: String::from(""),
                source_info: SourceInfo::init(1, 9, 1, 10),
            },
            expr: HurlString {
                value: String::from("xxx"),
                encoded: None,
                source_info: SourceInfo::init(1, 10, 1, 13),
            },
        },
    };

    let error = jsonpath_query.eval(json_http_response()).err().unwrap();
    assert_eq!(error.source_info.start, Pos { line: 1, column: 10 });
    assert_eq!(error.inner, RunnerError::QueryInvalidJsonpathExpression);
}


#[test]
fn test_query_invalid_json() {
    let http_response = http::response::Response {
        version: http::response::Version::Http10,
        status: 0,
        headers: vec![],
        body: String::into_bytes(String::from("xxx")),
    };
    let error = jsonpath_success().eval(http_response).err().unwrap();
    assert_eq!(error.source_info.start, Pos { line: 1, column: 1 });
    assert_eq!(error.inner, RunnerError::QueryInvalidJson);
}

#[test]
fn test_query_json_not_found() {
    let http_response = http::response::Response {
        version: http::response::Version::Http10,
        status: 0,
        headers: vec![],
        body: String::into_bytes(String::from("{}")),
    };
    //assert_eq!(jsonpath_success().eval(http_response).unwrap(), Value::List(vec![]));
    assert_eq!(jsonpath_success().eval(http_response).unwrap(), Value::None);
}

#[test]
fn test_query_json() {
    assert_eq!(
        jsonpath_success().eval(json_http_response()).unwrap(),
        Value::List(vec![Value::Bool(false)])
    );
    assert_eq!(
        jsonpath_errors().eval(json_http_response()).unwrap(),
        Value::List(vec![Value::List(vec![
            Value::Object(vec![(String::from("id"), Value::String(String::from("error1")))]),
            Value::Object(vec![(String::from("id"), Value::String(String::from("error2")))])
        ])])
    );
}

// endregion

