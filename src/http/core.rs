use serde::{Deserialize, Serialize};

use crate::runner::core::RunnerError;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

extern crate reqwest;


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


// region url

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
}

impl Url {
//    pub fn to_string(&self) -> String {
//        let port = match self.port {
//            None => String::from(""),
//            Some(p) => format!(":{}", p),
//        };
//        let mut s = format!("{}://{}{}{}", self.scheme, self.host, port, self.path);
//        match self.clone().querystring {
//            None => {}
//            Some(q) => s.push_str(format!("?{}", q).as_str())
//        }
//        return s;
//    }

//    pub fn eval(s: String) -> Result<Url, RunnerError> {
//        return match url::Url::parse(s.as_str()) {
//            Err(_) => Err(RunnerError::InvalidURL(s)),
//            Ok(u) => Ok(Url {
//                scheme: u.scheme().to_string(),
//                host: u.host_str().unwrap().to_string(),
//                port: u.port(),
//                path: u.path().to_string(),
//                querystring: match u.query() {
//                    None => None,
//                    Some(s) => Some(s.to_string())
//                },
//            })
//        };
//    }
}

//#[test]
//fn test_to_string() {
//    assert_eq!(hello_url().to_string(), String::from("http://localhost:8000/hello"))
//}


#[cfg(test)]
pub fn hello_url() -> Url {
    Url {
        scheme: String::from("http"),
        host: String::from("localhost"),
        port: Some(8000),
        path: String::from("/hello"),
    }
}

//endregion

// region header

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
}

pub fn get_header_value(headers: Vec<Header>, name: &str) -> Option<String> {
    for header in headers {
        if header.name == name.to_string() {
            return Some(header.value.clone());
        }
    }
    return None;
}

// endregion

// region param

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub value: String,
}



// region add_query_param
#[allow(dead_code)]
fn add_query_param(url: String, param: Param) -> String {
    let mut s = url.clone();
    if url.contains("?") {
        if !url.ends_with("?") {
            s.push_str("&");
        }
    } else {
        s.push_str("?")
    }
    let encoded = utf8_percent_encode(param.value.as_str(), FRAGMENT).to_string();
    s.push_str(format!("{}={}", param.name, encoded).as_str());
    return s;
}

#[test]
fn test_add_query_param() {
    let url = add_query_param(
        String::from("http://localhost:5000/querystring-params"),
        Param {
            name: String::from("param1"),
            value: String::from("value1"),
        },
    );
    assert_eq!(
        url,
        String::from("http://localhost:5000/querystring-params?param1=value1")
    );
    let url = add_query_param(
        url,
        Param {
            name: String::from("param2"),
            value: String::from(""),
        },
    );
    let url = add_query_param(
        url,
        Param {
            name: String::from("param3"),
            value: String::from("a b"),
        },
    );
    let url = add_query_param(
        url,
        Param {
            name: String::from("param4"),
            value: String::from("http://"),
        },
    );
    assert_eq!(
        url,
        String::from("http://localhost:5000/querystring-params?param1=value1&param2=&param3=a%20b&param4=http%3A%2F%2F")
    );
}

// endregion

// region add_form_param
#[allow(dead_code)]
pub fn encode_form_params(params: Vec<Param>) -> Vec<u8> {
    return params
        .iter()
        .map(|p| format!("{}={}", p.name, p.value))
        .collect::<Vec<_>>()
        .join("&")
        .into_bytes();
}

#[allow(dead_code)]
pub fn encode_form_params2(params: Vec<Param>) -> String {
    return params
        .iter()
        .map(|p| format!("{}={};", p.name, p.value))
        .collect::<Vec<_>>()
        .join("&");
}

#[test]
fn test_encode_form_params() {
    assert_eq!(
        encode_form_params(vec![
            Param {
                name: String::from("param1"),
                value: String::from("value1"),
            },
            Param {
                name: String::from("param2"),
                value: String::from(""),
            }
        ]),
        vec![
            112, 97, 114, 97, 109, 49, 61, 118, 97, 108, 117, 101, 49, 38, 112, 97, 114, 97, 109,
            50, 61
        ]
    );
}
// endregion


