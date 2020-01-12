#[cfg(test)]
use crate::core::core::{Pos, SourceInfo};
use crate::core::core::Value;

//use super::core::{Error, RunnerError};
use super::core::{Error};
use super::http;
use super::super::core::ast::*;

#[cfg(test)]
use self::super::query;
#[cfg(test)]
use super::core::{RunnerError};

pub type CaptureResult = Result<(String, Value), Error>;


impl Capture {
    pub fn eval(self, http_response: http::Response) -> CaptureResult {
        let value = self.query.clone().eval(http_response)?;
//        if !value.is_scalar() {
//            return Err(Error {
//                source_info: self.query.clone().source_info,
//                inner: RunnerError::CaptureNonScalarUnsupported,
//            }
//            );
//        }
        return Ok((self.name.value, value));
    }
}

// region test data


#[cfg(test)]
pub fn user_count_capture() -> Capture {

    // non scalar value
    let whitespace = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
    return Capture {
        line_terminators: vec![],
        space0: whitespace.clone(),
        name: HurlString {
            value: "UserCount".to_string(),
            encoded: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        space1: whitespace.clone(),
        space2: whitespace.clone(),

        // xpath count(//user)
        query: query::xpath_count_user_query(),
        line_terminator0: LineTerminator {
            space0: whitespace.clone(),
            comment: None,
            newline: whitespace.clone(),
        },
    };
}
// endregion

// region test status

#[test]
fn test_invalid_xpath() {
    let whitespace = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
    let capture = Capture {
        line_terminators: vec![],
        space0: whitespace.clone(),
        name: HurlString {
            value: "count".to_string(),
            encoded: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        space1: whitespace.clone(),
        space2: whitespace.clone(),

        query: query::xpath_invalid_query(),
        line_terminator0: LineTerminator {
            space0: whitespace.clone(),
            comment: None,
            newline: whitespace.clone(),
        },
    };

    let error = capture.eval(http::xml_three_users_http_response()).err().unwrap();
    assert_eq!(error.source_info.start, Pos { line: 1, column: 7 });
    assert_eq!(error.inner, RunnerError::QueryInvalidXpathEval)
}

#[test]
fn test_capture_unsupported() {

    // non scalar value
    let whitespace = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
    let _capture = Capture {
        line_terminators: vec![],
        space0: whitespace.clone(),
        name: HurlString {
            value: "???".to_string(),
            encoded: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        space1: whitespace.clone(),
        space2: whitespace.clone(),

        // xpath //user
        query: Query {
            source_info: SourceInfo::init(1, 1, 1, 13),
            value: QueryValue::Xpath {
                space0: whitespace.clone(),
                expr: HurlString {
                    value: "//user".to_string(),
                    encoded: None,
                    source_info: SourceInfo::init(1, 7, 1, 13),
                },
            },
        },
        line_terminator0: LineTerminator {
            space0: whitespace.clone(),
            comment: None,
            newline: whitespace.clone(),
        },
    };

    //println!("{:?}", capture.eval(xml_http_response()));
//    let error = capture.eval(http::xml_three_users_http_response()).err().unwrap();
//    assert_eq!(error.source_info.start, Pos { line: 1, column: 1 });
//    assert_eq!(error.inner, RunnerError::CaptureNonScalarUnsupported)
}


#[test]
fn test_capture() {
    assert_eq!(user_count_capture().eval(http::xml_three_users_http_response()).unwrap(),
               (String::from("UserCount"), Value::from_f64(3.0))
    );
}

// endregion
