use std::collections::HashMap;

#[cfg(test)]
use crate::core::core::SourceInfo;

use super::core::{Error, RunnerError};
use super::super::core::ast::*;

// region template

impl HurlTemplate {
    pub fn eval(self, variables: &HashMap<String, String>) -> Result<String, Error> {
        match self {
            HurlTemplate { elements, .. } => {
                let mut value = String::from("");
                for elem in elements {
                    match elem.eval(variables) {
                        Ok(v) => value.push_str(v.as_str()),
                        Err(e) => return Err(e),
                    }
                }
                return Ok(value);
            }
        };
    }
}

#[test]
fn test_template() {

// {{base_url}}/hello
    let space = Whitespace {
        value: String::from(""),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let template = HurlTemplate {
        elements: vec![
            HurlTemplateElement::Expression {
                value: Expr {
                    space0: space.clone(),
                    variable: Variable { name: String::from("base_url"), source_info: SourceInfo::init(1, 3, 1, 11) },
                    space1: space.clone(),
                }
            },
            HurlTemplateElement::Literal { value: HurlString2 { value: String::from("/hello"), encoded: None } }
        ],
        delimiter: "".to_string(),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };

    let mut variables = HashMap::new();
    variables.insert(String::from("base_url"), String::from("http://localhost:8000"));
    assert_eq!(template.clone().eval(&variables), Ok(String::from("http://localhost:8000/hello")));

    let variables = HashMap::new();
    let error = template.clone().eval(&variables).err().unwrap();
    assert_eq!(error.source_info, SourceInfo::init(1, 3, 1, 11));
    assert_eq!(error.inner, RunnerError::TemplateVariableNotDefined { name: String::from("base_url") });
}

// endregion

// region template-element
impl HurlTemplateElement {
    pub fn eval(self, variables: &HashMap<String, String>) -> Result<String, Error> {
        return match self {
            HurlTemplateElement::Literal { value: HurlString2 { value, .. } } => { Ok(value) }
            HurlTemplateElement::Expression { value: Expr { variable: Variable { name, source_info }, .. } } => {
                return match variables.get(&name as &str) {
                    Some(value) => Ok(value.to_string()),
                    _ => Err(Error { source_info, inner: RunnerError::TemplateVariableNotDefined { name }, assert: false }),
                };
            }
        };
    }
}


#[test]
fn test_template_element() {
    let mut variables = HashMap::new();
    variables.insert(String::from("base_url"), String::from("http://localhost:8000"));

    let template_element = HurlTemplateElement::Literal { value: HurlString2 { value: String::from("http://localhost:8000/hello"), encoded: None } };
    assert_eq!(template_element.eval(&variables), Ok(String::from("http://localhost:8000/hello")));

    let space = Whitespace {
        value: String::from(""),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let template_element = HurlTemplateElement::Expression {
        value: Expr {
            space0: space.clone(),
            variable: Variable { name: String::from("base_url"), source_info: SourceInfo::init(1, 1, 1, 10) },
            space1: space.clone(),
        }
    };
    assert_eq!(template_element.eval(&variables), Ok(String::from("http://localhost:8000")));
}

#[test]
fn test_template_element_error() {
    let variables = HashMap::new();
    let space = Whitespace {
        value: String::from(""),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let template_element = HurlTemplateElement::Expression {
        value: Expr {
            space0: space.clone(),
            variable: Variable { name: String::from("base_url"), source_info: SourceInfo::init(1, 1, 1, 10) },
            space1: space.clone(),
        }
    };
    let error = template_element.eval(&variables).err().unwrap();
    assert_eq!(error.source_info, SourceInfo::init(1, 1, 1, 10));
    assert_eq!(error.inner, RunnerError::TemplateVariableNotDefined { name: String::from("base_url") });
}
//endregion