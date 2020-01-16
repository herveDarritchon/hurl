use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use serde::{Deserialize, Serialize};

use super::cookie::*;
use super::core::*;

const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b':')
    .add(b'/')
    .add(b'<')
    .add(b'>')
    .add(b'+')
    .add(b'=')
    .add(b'`');

// region request
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub url: Url,
    pub querystring: Vec<Param>,
    pub headers: Vec<Header>,
    pub cookies: Vec<Cookie>,
    pub body: Vec<u8>,
}

fn has_header(headers: &Vec<Header>, name: String) -> bool {
    for header in headers {
        if header.name == name.to_string() {
            return true;
        }
    }
    return false;
}

impl Request {
    pub fn host(self) -> String {
        return self.url.host;
    }

    pub fn url(self) -> String {
        let port = match self.url.port {
            None => String::from(""),
            Some(p) => format!(":{}", p)
        };
        let querystring = if self.querystring.is_empty() {
            String::from("")
        } else {
            let mut buf = String::from("");
            for param in self.querystring {
                if !buf.is_empty() {
                    buf.push('&');
                }
                let encoded = utf8_percent_encode(param.value.as_str(), FRAGMENT).to_string();
                buf.push_str(format!("{}={}", param.name, encoded).as_str());
            }
            format!("?{}", buf)
        };
        return format!("{}://{}{}{}{}",
                       self.url.scheme,
                       self.url.host,
                       port,
                       self.url.path,
                       querystring
        );
    }

    pub fn headers(self) -> Vec<Header> {
        let mut headers: Vec<Header> = self.headers.clone();
        let user_agent = format!("hurl/{}", clap::crate_version!());
        let default_headers = vec![
            (String::from("User-Agent"), user_agent.clone()),
            (String::from("Host"), String::from(self.url.clone().host))
        ];

        for (name, value) in default_headers {
            if !has_header(&self.headers, name.clone()) {
                headers.push(Header { name, value });
            }
        }

        for cookie in self.cookies {
            headers.push(Header {
                name: String::from("Cookie"),
                value: cookie.to_string(),
            });
        }
        return headers;
    }

    pub fn add_session_cookies(&mut self, cookies: Vec<Cookie>) {
        eprintln!("add session cookies {:?}", cookies);
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
        },
        querystring: vec![],
        headers: vec![],
        cookies: vec![],
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
        },
        //String::from("http://localhost:8000/querystring-params"),
//        querystring_params: vec![
//            Param { name: String::from("param1"), value: String::from("value1") },
//            Param { name: String::from("param2"), value: String::from("") }
//        ],
        querystring: vec![
            Param { name: String::from("param1"), value: String::from("value1") },
            Param { name: String::from("param2"), value: String::from("a b") },
        ],
        headers: vec![
//            Header { name: String::from("User-Agent"), value: format!("hurl/{}", clap::crate_version!()) },
//            Header { name: String::from("Host"), value: String::from("localhost") }
        ],
        cookies: vec![],
        body: vec![],
    };
}


#[cfg(test)]
pub fn custom_http_request() -> Request {
    return Request {
        method: Method::Get,
        url: Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: None,
            path: "/custom".to_string(),
        },
        querystring: vec![],
        headers: vec![
            Header { name: String::from("User-Agent"), value: String::from("iPhone") },
            Header { name: String::from("Foo"), value: String::from("Bar") },
        ],
        cookies: vec![
            Cookie {
                name: String::from("theme"),
                value: String::from("light"),
                max_age: None,
                domain: None,
            },
            Cookie {
                name: String::from("sessionToken"),
                value: String::from("abc123"),
                max_age: None,
                domain: None,
            }
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


// region headers
#[test]
pub fn test_headers() {
    assert_eq!(hello_http_request().headers(), vec![
        Header { name: String::from("User-Agent"), value: format!("hurl/{}", clap::crate_version!()) },
        Header { name: String::from("Host"), value: String::from("localhost") }
    ]);

    assert_eq!(custom_http_request().headers(), vec![
        Header { name: String::from("User-Agent"), value: String::from("iPhone") },
        Header { name: String::from("Foo"), value: String::from("Bar") },
        Header { name: String::from("Host"), value: String::from("localhost") },
        Header { name: String::from("Cookie"), value: String::from("theme=light") },
        Header { name: String::from("Cookie"), value: String::from("sessionToken=abc123") },
    ]);
}


// endregion


// region url
#[test]
pub fn test_url() {
    assert_eq!(hello_http_request().url(), String::from("http://localhost:8000/hello"));
    assert_eq!(query_http_request().url(), String::from("http://localhost:8000/querystring-params?param1=value1&param2=a%20b"));
}


// endregion