//#[cfg(test)]
//use crate::core::core::Pos;
use std::collections::HashMap;

#[cfg(test)]
use crate::core::core::{SourceInfo, Value};
use crate::http;

use super::core::{Error, RunnerError};
use super::core::*;
#[cfg(test)]
use super::query;
use super::super::core::ast::*;

//#[cfg(test)]
//use crate::core::core::{Value, SourceInfo};

//#[cfg(test)]
//use crate::runner::core::{RunnerError};


impl AssertResult {
    pub fn fail(self) -> bool {
        return match self {
            AssertResult::Version { actual, expected, .. } => actual != expected,
            AssertResult::Status { actual, expected, .. } => actual != expected,
            AssertResult::Header { .. } => false,
            AssertResult::Explicit { .. } => true
        };
    }

    pub fn error(self) -> Option<Error> {
        return match self {
            AssertResult::Version { actual, expected, source_info } => {
                if actual == expected {
                    None
                } else {
                    Some(Error {
                        source_info,
                        inner: RunnerError::AssertVersion { actual: actual.to_string() }
                        ,
                        assert: false,
                    })
                }
            }
            AssertResult::Status { actual, expected, source_info } => {
                if actual == expected {
                    None
                } else {
                    Some(Error {
                        source_info,
                        inner: RunnerError::AssertStatus { actual: actual.to_string() },
                        assert: false,
                    })
                }
            }
            AssertResult::Header { actual, expected, source_info } => {
                match actual {
                    Err(e) => Some(e),
                    Ok(s) => {
                        if s == expected {
                            None
                        } else {
                            Some(Error {
                                source_info,
                                inner: RunnerError::AssertHeaderValueError { actual: s },
                                assert: false,
                            })
                        }
                    }
                }
            }
            AssertResult::Explicit { actual: Err(e), .. } => { Some(e) }
            AssertResult::Explicit { predicate_result: Some(Err(e)), .. } => { Some(e) }
            _ => None,
        };
    }
}


// region test data


#[cfg(test)]
// xpath //user countEquals 3
pub fn assert_count_user() -> Assert {
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let predicate = Predicate {
        not: false,
        space0: whitespace.clone(),
        predicate_func: PredicateFunc {
            source_info: SourceInfo::init(1, 14, 1, 27),
            value: PredicateFuncValue::CountEqual { space0: whitespace.clone(), value: 3 },
        },
    };
    return Assert {
        line_terminators: vec![],
        space0: whitespace.clone(),
        query: query::xpath_users(),
        space1: whitespace.clone(),
        predicate,
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
fn test_invalid_xpath() {}


impl Assert {
    pub fn eval(self, _variables: &HashMap<String, String>, http_response: http::response::Response) -> AssertResult {
        let actual = self.query.eval(http_response);
        let source_info = self.predicate.clone().predicate_func.source_info;
        let predicate_result = match actual.clone() {
            Err(_) => None,
            Ok(actual) => Some(self.predicate.eval(_variables, actual.clone()))
        };

        return AssertResult::Explicit { actual, source_info, predicate_result };
    }
}

#[test]
fn test_eval() {
    let variables = HashMap::new();
    assert_eq!(
        assert_count_user().eval(&variables, http::response::xml_three_users_http_response()),
        AssertResult::Explicit {
            actual: Ok(Value::Nodeset(3)),
            source_info: SourceInfo::init(1, 14, 1, 27),
            predicate_result: Some(Ok(())),
        }
    );

//    let error = assert_count_user().eval(&variables, http::xml_two_users_http_response()).err().unwrap();
//    assert_eq!(error.inner, RunnerError::PredicateValue(Value::Nodeset(2)));
//    assert_eq!(error.source_info, SourceInfo::init(1,14,1,27));
//
//    let error = assert_count_user().eval(&variables, http::xml_invalid_response()).err().unwrap();
//    assert_eq!(error.inner, RunnerError::QueryInvalidXml);
//    assert_eq!(error.source_info, SourceInfo::init(1,1,1,13));
}

// endregion