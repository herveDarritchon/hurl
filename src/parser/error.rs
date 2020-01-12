use crate::core::core::{FormatError, Pos, SourceInfo};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub pos: Pos,
    pub recoverable: bool,
    pub inner: ParseError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ParseError {
    Expecting { value: String},

    Method {},
    Version {},
    Status {},
    Filename {},
    Space {},
    SectionName{ name : String},
    JsonpathExpr {},
    XPathExpr {},
    TemplateVariable {},
    Json {},
    Xml {},
    Predicate,
    PredicateValue,

    Unexpected { character: String },
    Eof {},
    Url {},


}

#[allow(dead_code)]
impl FormatError for Error {
    fn source_info(&self) -> SourceInfo {
        return SourceInfo {
            start: self.pos.clone(),
            end: self.pos.clone(),
        };
    }

    fn description(&self) -> String {
        return match self.clone().inner {
            ParseError::Method { .. } => "Parsing Method".to_string(),
            ParseError::Version { .. } => "Parsing Version".to_string(),
            ParseError::Status { .. } => "Parsing Status".to_string(),
            ParseError::Filename { .. } => "Parsing Filename".to_string(),
            ParseError::Expecting { .. } => "Parsing literal".to_string(),
            ParseError::Space { .. } => "Parsing space".to_string(),
            ParseError::SectionName { .. } => "Parsing section name".to_string(),
            ParseError::JsonpathExpr { .. } => "Parsing jsonpath expression".to_string(),
            ParseError::XPathExpr { .. } => "Parsing xpath expression".to_string(),
            ParseError::TemplateVariable { .. } => "Parsing template variable".to_string(),
            ParseError::Json { .. } => "Parsing json".to_string(),
            ParseError::Predicate { .. } => "Parsing predicate".to_string(),
            ParseError::PredicateValue { .. } => "Parsing predicate value".to_string(),

            _ => format!("{:?}", self),
        };
    }

    fn fixme(&self) -> String {
        return match self.inner.clone() {
            ParseError::Method { .. } => "Available HTTP Method GET, POST, ...".to_string(),
            ParseError::Version { .. } => "The http version must be 1.0, 1.1, 2 or *".to_string(),
            ParseError::Status { .. } => "The http status is not valid".to_string(),
            ParseError::Filename { .. } => "expecting a filename".to_string(),
            ParseError::Expecting { value } => format!("expecting '{}'", value),
            ParseError::Space { .. } => "expecting a space".to_string(),
            ParseError::SectionName {name} =>  format!("the section {} is not valid", name),
            ParseError::JsonpathExpr { .. } => "expecting a jsonpath expression".to_string(),
            ParseError::XPathExpr { .. } => "expecting a xpath expression".to_string(),
            ParseError::TemplateVariable { .. } => "expecting a variable".to_string(),
            ParseError::Json { .. } => "json error".to_string(),
            ParseError::Predicate { .. } => "expecting a predicate".to_string(),
            ParseError::PredicateValue { .. } => "invalid predicate value".to_string(),

            _ => format!("{:?}", self),
        };
    }
}
