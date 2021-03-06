use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode, percent_decode};
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
    .add(b'?')
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

        if !self.cookies.is_empty() {
            headers.push(Header {
                name: String::from("Cookie"),
                value: self.cookies
                    .iter()
                    .map(|c| format!("{}={}", c.name, c.value))
                    .collect::<Vec<String>>()
                    .join("; "),
            });
        }
        return headers;
    }

    pub fn content_type(self) -> Option<String> {
        for Header { name, value } in self.headers {
            if name == String::from("Content-Type") {
                return Some(value);
            }
        }
        return None;
    }

    pub fn add_session_cookies(&mut self, cookies: Vec<Cookie>) {
        //eprintln!("add session cookies {:?}", cookies);

        for cookie in cookies {

            // TBC: both request and session cookies should have a domain => should not be an Option
            let session_domain = cookie.clone().domain.unwrap();
            match self.clone().get_cookie(cookie.clone().name) {
                Some(Cookie { domain: Some(domain), .. }) => {
                    if session_domain != domain {
                        self.cookies.push(cookie.clone());
                    }
                }
                _ => {
                    self.cookies.push(cookie.clone());
                }
            }
        }
    }


    pub fn get_cookie(self, name: String) -> Option<Cookie> {
        for cookie in self.cookies {
            if cookie.name == name {
                return Some(cookie);
            }
        }
        return None;
    }


    pub fn form_params(self) -> Option<Vec<Param>> {
        if self.clone().content_type() != Some(String::from("application/x-www-form-urlencoded")) {
            return None;
        }
        let decoded = percent_decode(&self.body);
        let params = match decoded.decode_utf8() {
            Ok(v) => {
                let params: Vec<&str> = v.split("&").collect();
                params.iter().map(|s| Param::parse(s)).collect()
            }
            _ => vec![]
        };
        return Some(params);
    }
}


impl Param {
    fn parse(s: &str) -> Param {
        match s.find('=') {
            None => Param { name: s.to_string(), value: String::from("") },
            Some(i) => {
                let (name, value) = s.split_at(i);
                Param { name: name.to_string(), value: value[1..].to_string() }
            }
        }
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
                path: None,
            },
            Cookie {
                name: String::from("sessionToken"),
                value: String::from("abc123"),
                max_age: None,
                domain: None,
                path: None,
            }
        ],
        body: vec![],
    };
}


#[cfg(test)]
pub fn form_http_request() -> Request {
    let params = vec![
        Param { name: String::from("param1"), value: String::from("value1") },
        Param { name: String::from("param2"), value: String::from("") },
        Param { name: String::from("param3"), value: String::from("a=b") },
    ];
    let encoded_params = params
        .iter()
        .map(|param| format!("{}={}", param.name, utf8_percent_encode(&param.value, FRAGMENT).to_string()))
        .collect::<Vec<String>>()
        .join("&");
    return Request {
        method: Method::Post,
        url: Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: None,
            path: "/form-params".to_string(),
        },
        querystring: vec![],
        headers: vec![
            Header { name: String::from("Content-Type"), value: String::from("application/x-www-form-urlencoded") },
        ],
        cookies: vec![],
        body: encoded_params.into_bytes(),
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

//    assert_eq!(custom_http_request().headers(), vec![
//        Header { name: String::from("User-Agent"), value: String::from("iPhone") },
//        Header { name: String::from("Foo"), value: String::from("Bar") },
//        Header { name: String::from("Host"), value: String::from("localhost") },
//        Header { name: String::from("Cookie"), value: String::from("theme=light") },
//        Header { name: String::from("Cookie"), value: String::from("sessionToken=abc123") },
//    ]);
    assert_eq!(custom_http_request().headers(), vec![
        Header { name: String::from("User-Agent"), value: String::from("iPhone") },
        Header { name: String::from("Foo"), value: String::from("Bar") },
        Header { name: String::from("Host"), value: String::from("localhost") },
        Header { name: String::from("Cookie"), value: String::from("theme=light; sessionToken=abc123") },
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


// region form_headers

// region headers

#[test]
pub fn test_form_params() {
    assert_eq!(hello_http_request().form_params(), None);
    assert_eq!(form_http_request().form_params().unwrap(), vec![
        Param { name: String::from("param1"), value: String::from("value1") },
        Param { name: String::from("param2"), value: String::from("") },
        Param { name: String::from("param3"), value: String::from("a=b") },
    ]);
}

// endregion
