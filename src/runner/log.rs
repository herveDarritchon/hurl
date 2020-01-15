use serde::ser::{Serializer, SerializeStruct};
use serde::Serialize;

use crate::http;

use super::core::*;

impl Serialize for HurlResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("filename", &self.clone().filename)?;
        state.serialize_field("entries", &self.clone().entries)?;
        state.end()
    }
}

impl Serialize for EntryResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("request", &self.request)?;
        state.serialize_field("response", &self.response)?;
        state.serialize_field("captures", &self.captures)?;
        state.serialize_field("asserts", &self.asserts)?;

        state.end()
    }
}

impl Serialize for AssertResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("??", 3)?;
        match self {
            AssertResult::Version { source_info, actual, expected } => {
                state.serialize_field("source_info", source_info)?;
                state.serialize_field("actual", actual)?;
                state.serialize_field("expected", expected)?;
            }
            _ => {}
        };
        state.end()
    }
}

// http-specific

impl Serialize for http::request::Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("url", &self.clone().url())?;
        state.serialize_field("queryString", &self.clone().querystring)?;
        state.serialize_field("headers", &self.clone().headers())?;
        state.serialize_field("cookies", &self.clone().cookies)?;
        state.end()
    }
}

impl Serialize for http::response::Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("httpVersion", &self.clone().version)?;
        state.serialize_field("status", &self.clone().status)?;
        state.serialize_field("cookies", &self.clone().cookies())?;
        state.serialize_field("headers", &self.clone().headers)?;

        state.end()
    }
}

impl Serialize for http::cookie::Cookie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("name", &self.clone().name)?;
        state.serialize_field("value", &self.clone().value)?;
        if let Some(value) = self.clone().domain {
            state.serialize_field("domain", &value)?;
        }
        state.end()
    }
}

impl Serialize for http::response::Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            http::response::Version::Http10 => serializer.serialize_str("HTTP/1.0"),
            http::response::Version::Http11 => serializer.serialize_str("HTTP/1.1"),
            http::response::Version::Http2 => serializer.serialize_str("HTTP/2"),
        }
    }
}



