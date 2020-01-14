use std::collections::HashMap;

use crate::core::core::Value;
//#[cfg(test)]
use crate::runner::core::RunnerError;
#[cfg(test)]
use crate::core::core::SourceInfo;

use super::core::Error;
//use super::http;
use super::super::core::ast::*;

#[cfg(test)]
use self::super::assert;
use self::super::assert::AssertResult;
#[cfg(test)]
use self::super::capture;

use crate::http;
use crate::runner::text::Textable;

//#[cfg(test)]
//use crate::core::core::{SourceInfo};

pub type ResponseResult = Result<ResponseLog, Error>;

pub struct ResponseLog {
    //value: http::Response,
    _captures: Vec<(String, Value)>,
    _asserts: Vec<AssertResult>,
}


impl Response {
//    pub fn eval(self, variables: &mut HashMap<String, String>, http_response: http::Response) -> ResponseResult {
//        let mut assert_results = vec![];
//        for assert in self.clone().asserts() {
//            assert_results.push(assert.eval(variables, http_response.clone()));
//        }
//        for Capture { name, query, .. } in self.clone().captures() {
//            let value = query.eval(http_response.clone())?;
//            variables.insert(name.value, value.to_string());
//        }
//        return Ok(ResponseLog {
//           captures: vec![],
//           asserts: assert_results
//        });
//    }

    //pub fn eval_asserts(self, _variables: &HashMap<String, String>, http_response: http::Response) -> Result<Vec<AssertResult>, Error> {
    pub fn eval_asserts(self, _variables: &HashMap<String, String>, http_response: http::response::Response) -> Vec<AssertResult> {
        let mut asserts = vec![];

        let version = self.clone().version;
        asserts.push(AssertResult::Version {
            actual: http_response.clone().version.to_text(),
            expected: version.value.as_str().to_string(),
            source_info: version.source_info
        });

        let status = self.clone().status;
        asserts.push(AssertResult::Status {
            actual: http_response.clone().status as u64,
            expected: status.value as u64,
            source_info: status.source_info,
        });

        for header in self.clone().headers {
            //eprintln!(">> header {:?}", header);
            match header.value.clone().eval(_variables) {
                Err(e) => {
                    asserts.push(AssertResult::Header {
                        actual: Err(e),
                        expected: String::from(""),
                        source_info: header.name.clone().source_info,
                    });
                }
                Ok(expected) => {
                    match http_response.get_header(header.name.value.clone().as_str(), false) {
                        None => {
                            asserts.push(AssertResult::Header {
                                actual: Err(Error {
                                    source_info: header.name.clone().source_info,
                                    inner: RunnerError::QueryHeaderNotFound {},
                                    assert: false
                                }),
                                expected,
                                source_info: header.name.clone().source_info,
                            });
                        }
                        Some(actual) => {
                            asserts.push(AssertResult::Header {
                                actual: Ok(actual),
                                expected,
                                source_info: header.value.clone().source_info,
                            });
                        }
                    }
                }
            }
        }
        for assert in self.asserts() {
            let assert_result = assert.eval(_variables, http_response.clone());
            //eprintln!(">> assert {:#?}", assert_result.clone());
            asserts.push(assert_result);
        }
        asserts
    }

    pub fn eval_captures(self, _variables: &HashMap<String, String>, http_response: http::response::Response) -> Result<Vec<(String, Value)>, Error> {
        let mut captures = vec![];
        for Capture { name, query, .. } in self.clone().captures() {
            let value = query.eval(http_response.clone())?;
            captures.push((name.value, value));
        }
        Ok(captures)
    }


}


#[cfg(test)]
pub fn user_response() -> Response {
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let line_terminator = LineTerminator {
        space0: whitespace.clone(),
        comment: None,
        newline: whitespace.clone(),
    };
    // HTTP/1.1 200
    let response = Response {
        line_terminators: vec![],
        version: Version { value: VersionValue::Version1, source_info: SourceInfo:: init(2,6,2,9)},
        space0: whitespace.clone(),
        status: Status { value: 200, source_info: SourceInfo::init(2, 10, 2, 13) },
        space1: whitespace.clone(),
        line_terminator0: line_terminator.clone(),
        headers: vec![],
        sections: vec![
            Section {
                line_terminators: vec![],
                space0: whitespace.clone(),
                line_terminator0: line_terminator.clone(),
                value: SectionValue::Asserts(vec![
                    assert::assert_count_user(),
                ]),
            },
            Section {
                line_terminators: vec![],
                space0: whitespace.clone(),
                line_terminator0: line_terminator.clone(),
                value: SectionValue::Captures(vec![
                    capture::user_count_capture(),
                ]),
            }
        ],
        body: None,
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    response
}

//#[test]
//pub fn test_response() {
//    let mut variables = HashMap::new();
//    //println!("{:?}", response.eval(&mut variables, http::xml_two_users_http_response()));
//    let asserts = response.eval(&mut variables, http::xml_two_users_http_response()).unwrap();
//    let first_assert_error = asserts.get(0).unwrap().as_ref().err().unwrap();
//    assert_eq!(first_assert_error.inner, RunnerError::PredicateValue(Value::Nodeset(2)));
//    assert_eq!(first_assert_error.source_info, SourceInfo::init(1, 14, 1, 27));
//    println!("{:?}", first_assert_error);
//    //let error = response.eval(&mut variables, http::xml_two_users_http_response()).err().unwrap();
//    //assert_eq!(error.inner, RunnerError::PredicateValue(Value::Nodeset(2)));
//    println!("{:?}", variables);
//    assert_eq!(variables.get("UserCount").unwrap(), "2.0");
//}

#[test]
pub fn test_eval_asserts() {
    let variables = HashMap::new();
    assert_eq!(
        user_response().eval_asserts(&variables, http::response::xml_two_users_http_response()),
        vec![
            AssertResult::Version {
                actual: String::from("1.0"),
                expected: String::from("1.0"),
                source_info: SourceInfo::init(2, 6, 2, 9)
            },
            AssertResult::Status {
                actual: 200,
                expected: 200,
                source_info: SourceInfo::init(2, 10, 2, 13)
            },
            AssertResult::Explicit {
                actual: Ok(Value::Nodeset(2)),
                source_info: SourceInfo::init(1, 14, 1, 27),
                predicate_result: Some(Err(Error {
                    source_info: SourceInfo::init(1, 14, 1, 27),
                    inner: RunnerError::PredicateValue(Value::Nodeset(2)),
                    assert: false
                })),
            }
        ]
    );

    //println!("{:?}", response.eval(&mut variables, http::xml_two_users_http_response()));
//    let asserts = response.eval(&mut variables, http::xml_two_users_http_response()).unwrap();
//    let first_assert_error = asserts.get(0).unwrap().as_ref().err().unwrap();
//    assert_eq!(first_assert_error.inner, RunnerError::PredicateValue(Value::Nodeset(2)));
//    assert_eq!(first_assert_error.source_info, SourceInfo::init(1, 14, 1, 27));
//    println!("{:?}", first_assert_error);
//    //let error = response.eval(&mut variables, http::xml_two_users_http_response()).err().unwrap();
//    //assert_eq!(error.inner, RunnerError::PredicateValue(Value::Nodeset(2)));
//    println!("{:?}", variables);
//    assert_eq!(variables.get("UserCount").unwrap(), "2.0");
}

#[test]
pub fn test_eval_captures() {
    let variables = HashMap::new();
    assert_eq!(
        user_response().eval_captures(&variables, http::response::xml_two_users_http_response()).unwrap(),
        vec![
            (String::from("UserCount"), Value::Float(2, 0))
        ]
    );
}