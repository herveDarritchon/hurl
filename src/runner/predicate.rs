use std::collections::HashMap;

use regex::Regex;

//use serde::{Deserialize, Serialize};
#[cfg(test)]
use crate::core::core::SourceInfo;
use crate::core::core::Value;

use super::core::{Error, RunnerError};
use super::core::*;
use super::super::core::ast::*;

// equals 10         function  return ()
// not equals 10
// countEquals 3               return () => ok        PredicateExpectedError
// not countEquals                           nok

// PredicateValue   => Recoverable with a not
// PredicateType


// xpath boolean(//user) equals 10
//                       ^^^^^^^^^^   Type does not matched with value return by query (generic message for the time-being
// xpath boolean(//user) not equals 10
//                       ^^^^^^^^^^^^^   Type does not matched with value return by query
// xpath cont(//user)  equals 10
//                     ^^^^^^^^^^^^^   actual value is 9
// xpath cont(//user)  greaterThan 10
//                     ^^^^^^^^^^^^^^   actual value is 9

// Predicate
// 2 evals


// 1) eval template
// 2) eval predicate

// equals template  becomes and equals string


impl Predicate {
    pub fn eval(self, variables: &HashMap<String, String>, value: Value) -> PredicateResult {
        return match self.predicate_func.clone().eval(variables, value.clone()) {
            Ok(_) => {
                if self.not {
                    Err(Error {
                        source_info: self.predicate_func.source_info,
                        inner: RunnerError::PredicateValue(value),
                        assert: false,
                    })
                } else { Ok(()) }
            }
            Err(Error { inner: RunnerError::PredicateValue(_), .. }) => {
                if self.not {
                    Ok(())
                } else {
                    Err(Error {
                        source_info: self.predicate_func.source_info,
                        inner: RunnerError::PredicateValue(value),
                        assert: false,
                    })
                }
            }
            Err(e) => Err(e)
        };
    }
}
// region test data


// endregion

// region test status

#[test]
fn test_invalid_xpath() {}


#[test]
fn test_predicate() {

    // not equals 10 with value 1     OK
    // not equals 10 with value 10    ValueError
    // not equals 10 with value true  TypeError
    let variables = HashMap::new();
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };

    let predicate = Predicate {
        not: true,
        space0: whitespace.clone(),
        predicate_func: PredicateFunc {
            value: PredicateFuncValue::EqualInt { space0: whitespace.clone(), value: 10 },
            source_info: SourceInfo::init(1, 5, 1, 14),
        },
    };

    let error = predicate.clone().eval(&variables, Value::Bool(true)).err().unwrap();
    assert_eq!(error.inner, RunnerError::PredicateType {});
    assert_eq!(error.source_info, SourceInfo::init(1, 5, 1, 14));


    let error = predicate.clone().eval(&variables, Value::Integer(10)).err().unwrap();
    assert_eq!(error.inner, RunnerError::PredicateValue(Value::Integer(10)));
    assert_eq!(error.source_info, SourceInfo::init(1, 5, 1, 14));

    assert_eq!(predicate.clone().eval(&variables, Value::Integer(1)).unwrap(), ());
}

// endregion


// region predicate-func
// no source_info involved


impl PredicateFunc {
    pub fn eval(self, variables: &HashMap<String, String>, value: Value) -> Result<(), Error> {
//        eprintln!(">>> actual={:?}", value);
//        eprintln!(">>> predicate={:#?}", self.clone());
        let source_info = self.source_info;
        return match (self.value, value.clone()) {

            // equals integer
            (PredicateFuncValue::EqualInt { value: expected, .. }, Value::Integer(actual)) =>
                if actual == expected { Ok(()) } else { Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false }) }
            (PredicateFuncValue::EqualInt { value: expected, .. }, Value::Float(int, decimal)) =>
                if int == expected && decimal == 0 { Ok(()) } else { Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false }) }

            // equals boolean
            (PredicateFuncValue::EqualBool { value: expected, .. }, Value::Bool(actual)) =>
                if actual == expected { Ok(()) } else { Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false }) }

            // equals float
            (PredicateFuncValue::EqualFloat { value: Float { int: expected_int, decimal: expected_dec, .. }, .. }, Value::Float(int, decimal)) => {
                if int == expected_int && decimal == expected_dec {
                    Ok(())
                } else {
                    Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false })
                }
            }
            (PredicateFuncValue::EqualFloat { value: Float { int: expected_int, decimal: expected_dec, .. }, .. }, Value::Integer(actual)) => {
                if actual == expected_int && expected_dec == 0 {
                    Ok(())
                } else {
                    Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false })
                }
            }

            // equals string
            (PredicateFuncValue::EqualString { value: template, .. }, Value::String(actual)) => {
                let expected = template.eval(variables)?;
//                eprintln!(">> expected={}", expected);
//                eprintln!(">>   actual={}", actual);
                if actual == expected {
                    return Ok(());
                } else {
                    return Err(Error { source_info, inner: RunnerError::PredicateValue(Value::String(expected)), assert: false });
                }
            }

            // startswith string
            (PredicateFuncValue::StartWith { value: template, .. }, Value::String(actual)) => {
                let value = template.eval(variables)?;
                if actual.as_str().starts_with(value.as_str()) {
                    return Ok(());
                } else {
                    return Err(Error { source_info, inner: RunnerError::PredicateValue(Value::String(value)), assert: false });
                }
            }

            // contains string
            (PredicateFuncValue::Contain { value: template, .. }, Value::String(actual)) => {
                let value = template.eval(variables)?;
                if actual.as_str().contains(value.as_str()) {
                    return Ok(());
                } else {
                    return Err(Error { source_info, inner: RunnerError::PredicateValue(Value::String(value)), assert: false });
                }
            }

            // match regex
            (PredicateFuncValue::Match { value, .. }, Value::String(actual)) => {
                match Regex::new(value.value.as_str()) {
                    Ok(re) => if re.is_match(actual.as_str()) {
                        Ok(())
                    } else {
                        return Err(Error { source_info, inner: RunnerError::PredicateValue(Value::String(actual)), assert: false });
                    }
                    _ => Err(Error { source_info: value.source_info, inner: RunnerError::InvalidRegex(), assert: false })
                }
            }


            // countEquals integer
            (PredicateFuncValue::CountEqual {
                value: expected, ..
            }, Value::List(values)) =>
                if values.len() as u64 == expected { Ok(()) } else { Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false }) }

            (PredicateFuncValue::CountEqual { value: expected, .. }, Value::Nodeset(length)) =>
                if length as u64 == expected {
                    Ok(())
                } else {
                    Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false })
                },


            // firstEquals bool
            (PredicateFuncValue::FirstEqualBool { value: expected, .. }, Value::List(values)) => {
                match values.get(0) {
                    Some(Value::Bool(actual)) => {
                        if *actual == expected {
                            Ok(())
                        } else {
                            Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false })
                        }
                    }
                    _ => return Err(Error { source_info, inner: RunnerError::PredicateType, assert: false })
                }
            }

            // firstEquals String
            (PredicateFuncValue::FirstEqualString { value: expected, .. }, Value::List(values)) => {
                let expected = expected.eval(variables)?;
                match values.get(0) {
                    Some(Value::String(actual)) => {
                        if *actual == expected {
                            Ok(())
                        } else {
                            Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false })
                        }
                    }
                    _ => return Err(Error { source_info, inner: RunnerError::PredicateType, assert: false })
                }
            }

            // firstEquals Int
            (PredicateFuncValue::FirstEqualInt { value: expected, .. }, Value::List(values)) => {
                match values.get(0) {
                    Some(Value::Integer(actual)) => {
                        if *actual == expected {
                            Ok(())
                        } else {
                            Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false })
                        }
                    }
                    _ => return Err(Error { source_info, inner: RunnerError::PredicateType, assert: false })
                }
            }

            // firstCountEqual
            (PredicateFuncValue::FirstCountEqual { value: expected, .. }, Value::List(values)) => {
                match values.get(0) {
                    Some(Value::List(values)) => if values.len() as u64 == expected { Ok(()) } else { Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false }) }
                    _ => return Err(Error { source_info, inner: RunnerError::PredicateType, assert: false })
                }
            }


            // exist
            (PredicateFuncValue::Exist {}, value) => {
                match value {
                    Value::None | Value::Nodeset(0) => Err(Error { source_info, inner: RunnerError::PredicateValue(value), assert: false }),
                    _ => Ok(())
                }
            }


            // default
            _ => Err(Error { source_info, inner: RunnerError::PredicateType, assert: false }),
        };
    }
}


#[test]
fn test_predicate_type_error() {
    let variables = HashMap::new();
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let error = PredicateFunc {
        value: PredicateFuncValue::EqualInt { space0: whitespace.clone(), value: 10 },
        source_info: SourceInfo::init(0, 0, 0, 0),
    }.eval(&variables, Value::Bool(true)).err().unwrap();
    assert_eq!(error.inner, RunnerError::PredicateType);
}

#[test]
fn test_predicate_value_error() {
    let variables = HashMap::new();
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let actual = Value::Integer(1);
    let error = PredicateFunc {
        source_info: SourceInfo::init(0, 0, 0, 0),
        value: PredicateFuncValue::EqualInt { space0: whitespace.clone(), value: 2 },
    }.eval(&variables, actual.clone()).err().unwrap();
    assert_eq!(error.inner, RunnerError::PredicateValue(actual.clone()));

    let actual = Value::Bool(true);
    let error = PredicateFunc {
        source_info: SourceInfo::init(0, 0, 0, 0),
        value: PredicateFuncValue::EqualBool { space0: whitespace.clone(), value: false },
    }.eval(&variables, actual.clone()).err().unwrap();
    assert_eq!(error.inner, RunnerError::PredicateValue(actual.clone()));

    let actual = Value::Float(1, 1);
    let error = PredicateFunc {
        source_info: SourceInfo::init(0, 0, 0, 0),
        value: PredicateFuncValue::EqualFloat { space0: whitespace.clone(), value: Float { int: 1, decimal: 200000000000000000, decimal_digits: 0 } },
    }.eval(&variables, actual.clone()).err().unwrap();
    assert_eq!(error.inner, RunnerError::PredicateValue(actual.clone()));
}


#[test]
fn test_predicate_value_equals() {
    let variables = HashMap::new();
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    assert_eq!(PredicateFunc {
        value: PredicateFuncValue::EqualInt { space0: whitespace.clone(), value: 10 },
        source_info: SourceInfo::init(0, 0, 0, 0),
    }.eval(&variables, Value::Integer(10)).unwrap(), ());

    assert_eq!(PredicateFunc {
        value: PredicateFuncValue::EqualBool { space0: whitespace.clone(), value: true },
        source_info: SourceInfo::init(0, 0, 0, 0),
    }.eval(&variables, Value::Bool(true)).unwrap(), ());

    assert_eq!(PredicateFunc {
        value: PredicateFuncValue::EqualFloat { space0: whitespace.clone(), value: Float { int: 1, decimal: 100000000000000000, decimal_digits: 0 } },
        source_info: SourceInfo::init(0, 0, 0, 0),
    }.eval(&variables, Value::Float(1, 100000000000000000)).unwrap(), ());


    // int and float => no type error
    assert_eq!(PredicateFunc {
        source_info: SourceInfo::init(0, 0, 0, 0),
        value: PredicateFuncValue::EqualInt { space0: whitespace.clone(), value: 1 },
    }.eval(&variables, Value::Float(1, 0)).unwrap(), ());

    assert_eq!(PredicateFunc {
        value: PredicateFuncValue::EqualFloat { space0: whitespace.clone(), value: Float { int: 1, decimal: 0, decimal_digits: 0 } },
        source_info: SourceInfo::init(0, 0, 0, 0),
    }.eval(&variables, Value::Integer(1)).unwrap(), ());
}

#[test]
fn test_predicate_value_equals_string() {
    let mut variables = HashMap::new();
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };

    // equals "{{base_url}}"
    let template = HurlTemplate {
        elements: vec![
            HurlTemplateElement::Expression {
                value: Expr {
                    space0: Whitespace { value: "".to_string(), source_info: SourceInfo::init(1, 11, 1, 11) },
                    variable: Variable { name: String::from("base_url"), source_info: SourceInfo::init(1, 11, 1, 19) },
                    space1: Whitespace { value: "".to_string(), source_info: SourceInfo::init(1, 19, 1, 19) },
                }
            }
        ],
        delimiter: "\"".to_string(),
        source_info: SourceInfo::init(1, 1, 1, 1),
    };

    let error = PredicateFunc {
        value: PredicateFuncValue::EqualString { space0: whitespace.clone(), value: template.clone() },
        source_info: SourceInfo::init(1, 1, 1, 21),
    }.eval(&variables, Value::String(String::from("http://localhost:8000"))).err().unwrap();
    assert_eq!(error.inner, RunnerError::TemplateVariableNotDefined { name: String::from("base_url") });
    assert_eq!(error.source_info, SourceInfo::init(1, 11, 1, 19));

    variables.insert(String::from("base_url"), String::from("http://localhost:8000"));
    assert_eq!(
        PredicateFunc {
            value: PredicateFuncValue::EqualString { space0: whitespace.clone(), value: template.clone() },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }.eval(&variables, Value::String(String::from("http://localhost:8000"))).unwrap(),
        ()
    );

    // assert_eq!(1, 2);
}

#[test]
fn test_predicate_count_equals_error() {
    let variables = HashMap::new();
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let error = PredicateFunc {
        value: PredicateFuncValue::CountEqual { space0: whitespace.clone(), value: 10 },
        source_info: SourceInfo::init(0, 0, 0, 0),
    }.eval(&variables, Value::Bool(true)).err().unwrap();
    assert_eq!(error.inner, RunnerError::PredicateType);


    let actual = Value::List(vec![]);
    let error = PredicateFunc {
        value: PredicateFuncValue::CountEqual { space0: whitespace.clone(), value: 1 },
        source_info: SourceInfo::init(0, 0, 0, 0),
    }.eval(&variables, actual.clone()).err().unwrap();
    assert_eq!(error.inner, RunnerError::PredicateValue(actual.clone()));

    let actual = Value::Nodeset(3);
    let error = PredicateFunc {
        source_info: SourceInfo::init(0, 0, 0, 0),
        value: PredicateFuncValue::CountEqual { space0: whitespace.clone(), value: 1 },
    }.eval(&variables, actual.clone()).err().unwrap();
    assert_eq!(error.inner, RunnerError::PredicateValue(actual.clone()));
}

#[test]
fn test_predicate_count_equals() {
    let variables = HashMap::new();
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    assert_eq!(PredicateFunc {
        value: PredicateFuncValue::CountEqual { space0: whitespace.clone(), value: 1 },
        source_info: SourceInfo::init(0, 0, 0, 0),
    }.eval(&variables, Value::List(vec![Value::Integer(1)])).unwrap(), ());
    assert_eq!(PredicateFunc {
        value: PredicateFuncValue::CountEqual { space0: whitespace.clone(), value: 1 },
        source_info: SourceInfo::init(0, 0, 0, 0),
    }.eval(&variables, Value::Nodeset(1)).unwrap(), ());
}

//#[test]
//fn test_predicate_starts_with() {
//    let actual = Value::List(vec![]);
//    let error = PredicateFunc::StartWith { space0: whitespace.clone(), value: "http://" }.eval(actual.clone()).err().unwrap();
//    assert_eq!(error, RunnerError::PredicateValue(actual.clone()));
//
//}
// endregion