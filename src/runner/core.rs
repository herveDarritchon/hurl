use crate::core::core::{FormatError, SourceInfo, Value};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Error {
    pub source_info: SourceInfo,
    pub inner: RunnerError,
    pub assert: bool
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum RunnerError {
    TemplateVariableNotDefined { name: String },
    InvalidURL(String),
    HttpConnection { url: String, message: String },
    FileReadAccess { value: String },

    // Capture
    //CaptureNonScalarUnsupported,
    //??CaptureError {},

    // Query
    QueryHeaderNotFound,
    QueryCookieNotFound,
    QueryInvalidJsonpathExpression,
    QueryInvalidXpathEval,
    QueryInvalidXml,
    QueryInvalidJson,
    QueryInvalidUtf8,

    // Predicate
    PredicateType,
    PredicateValue(Value),
    InvalidRegex(),


    AssertHeaderValueError { actual: String },
    AssertVersion { actual : String },
    AssertStatus { actual : String },

}

#[allow(dead_code)]
impl FormatError for Error {
    fn source_info(&self) -> SourceInfo {
        return self.clone().source_info;
    }

    fn description(&self) -> String {
        return match &self.inner {
            RunnerError::InvalidURL(..) => format!("Invalid url"),
            RunnerError::TemplateVariableNotDefined { .. } => format!("Undefined Variable"),
            RunnerError::HttpConnection { .. } => format!("Http Connection"),
            RunnerError::PredicateValue { .. } => format!("Assert - Predicate Value Failed"),
            RunnerError::InvalidRegex {} => format!("Invalid regex"),
            RunnerError::FileReadAccess {..} => format!("File ReadAccess"),
            RunnerError::QueryInvalidXml {..} => format!("Invalid XML"),
            RunnerError::QueryInvalidXpathEval {}=> format!("Invalid xpath expression"),
            RunnerError::QueryHeaderNotFound {}=> format!("Header not Found"),
            RunnerError::QueryCookieNotFound {}=> format!("Cookie not Found"),
            RunnerError::AssertHeaderValueError {..}=> format!("Assert Header Value"),
            RunnerError::AssertVersion {..}=> format!("Assert Http Version"),
            RunnerError::AssertStatus {..}=> format!("Assert Status"),
            RunnerError::QueryInvalidJson {..}=> format!("Invalid Json"),
            RunnerError::QueryInvalidUtf8 {..}=> format!("Invalid Utf8"),
            RunnerError::QueryInvalidJsonpathExpression {..} => format!("Invalid jsonpath"),
            RunnerError::PredicateType { .. } => format!("Assert - Inconsistent predicate type"),
        };
    }

    fn fixme(&self) -> String {
        return match &self.inner {
            RunnerError::InvalidURL(url) => format!("Invalid url '{}'", url),
            RunnerError::TemplateVariableNotDefined { name } => format!("You must set the variable {}", name),
            RunnerError::HttpConnection { url, message } => format!("can not connect to {} ({})", url, message),
            RunnerError::AssertVersion { actual, .. } => format!("actual value is {}", actual),
            RunnerError::AssertStatus { actual, .. } => format!("actual value is {}", actual),
            RunnerError::PredicateValue(value) => format!("actual value is {}", value.to_string()),
            RunnerError::InvalidRegex {} => format!("Regex expression is not valid"),
            RunnerError::FileReadAccess {value} => format!("File {} can not be read", value),
            RunnerError::QueryInvalidXml {..} => format!("The Http response is not a valid XML"),
            RunnerError::QueryHeaderNotFound {}=> format!("This header has not been found in the response"),
            RunnerError::QueryCookieNotFound {}=> format!("This cookie has not been found in the response"),
            RunnerError::QueryInvalidXpathEval {}=> format!("The xpath expression is not valid"),
            RunnerError::AssertHeaderValueError {actual}=> format!("actual value is {}", actual),
            RunnerError::QueryInvalidJson {..}=> format!("The http response is not a valid json"),
            RunnerError::QueryInvalidUtf8 {..}=> format!("The http response is not a valid utf8 string"),
            RunnerError::QueryInvalidJsonpathExpression {..} => format!("the jsonpath xpression is not valid"),
            RunnerError::PredicateType { .. } => format!("predicate type inconsistent with value return by query"),
        };
    }
}
