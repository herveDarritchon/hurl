use super::core::*;
use serde::{Deserialize, Serialize};

// region request
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub method: Method,
    pub url: Url,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
}


impl Request {
    pub fn host(self) -> String {
        return self.url.host;
    }
}

#[cfg(test)]
pub fn hello_http_request() -> Request {
    return Request {
        method: Method::Get,
        url: Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/hello".to_string(),
            querystring: None,
        },
        //String::from("http://localhost:8000/hello"),

        headers: vec![
            Header { name: String::from("User-Agent"), value: format!("hurl/{}", clap::crate_version!()) },
            Header { name: String::from("Host"), value: String::from("localhost") }
        ],
        body: vec![],
    };
}


// GET http://localhost:8000/querystring-params?param1=value1&param2
#[cfg(test)]
pub fn query_http_request() -> Request {
    return Request {
        method: Method::Get,
        url: Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/querystring-params".to_string(),
            querystring: Some(String::from("param1=value1&param2=")),
        },
        //String::from("http://localhost:8000/querystring-params"),
//        querystring_params: vec![
//            Param { name: String::from("param1"), value: String::from("value1") },
//            Param { name: String::from("param2"), value: String::from("") }
//        ],
        headers: vec![
            Header { name: String::from("User-Agent"), value: format!("hurl/{}", clap::crate_version!()) },
            Header { name: String::from("Host"), value: String::from("localhost") }
        ],
        body: vec![],
    };
}

// endregion

// region method
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

impl Method {
    pub fn to_reqwest(self) -> reqwest::Method {
        return match self {
            Method::Get => reqwest::Method::GET,
            Method::Head => reqwest::Method::HEAD,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Connect => reqwest::Method::CONNECT,
            Method::Options => reqwest::Method::OPTIONS,
            Method::Trace => reqwest::Method::TRACE,
        };
    }
}
// endregion