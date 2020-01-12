use crate::core::core::{FormatError, SourceInfo};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub source_info: SourceInfo,
    pub inner: LinterError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LinterError {
    UnneccessarySpace {},
    UnneccessaryJsonEncoding {},
    OneSpace {},
}

#[allow(dead_code)]
impl FormatError for Error {
    fn source_info(&self) -> SourceInfo {
        return self.clone().source_info;
    }

    fn description(&self) -> String {
        return match self.inner {
            LinterError::UnneccessarySpace { .. } => "Unnecessary space".to_string(),
            LinterError::UnneccessaryJsonEncoding {} => "Unnecessary json encoding".to_string(),
            LinterError::OneSpace {} => "One space ".to_string(),
        };
    }

    fn fixme(&self) -> String {
        return match self.inner {
            LinterError::UnneccessarySpace { .. } => "Remove space".to_string(),
            LinterError::UnneccessaryJsonEncoding {} => "Use Simple String".to_string(),
            LinterError::OneSpace {} => "Use only one space".to_string(),
        };
    }
}

pub trait Lintable<T> {
    fn errors(&self) -> Vec<Error>;
    fn lint(&self) -> T;
}
