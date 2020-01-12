extern crate libxml;
extern crate serde_json;
extern crate url;

use std::collections::HashMap;

#[cfg(test)]
use crate::core::core::SourceInfo;

use super::core::{Error, RunnerError};
use super::http;
//use super::log::*;
use super::super::core::ast::*;

fn has_header(headers: &Vec<http::Header>, name: String) -> bool {
    for header in headers {
        if header.name == name.to_string() {
            return true;
        }
    }
    return false;
}

// region request
impl Request {
    pub fn eval(self,
                variables: &HashMap<String, String>,
                current_cookies: &HashMap<String, http::Cookie>,
                context_dir: &str,
    )
                -> Result<http::Request, Error> {


        //
        // calculate url
        //
        let url = self.clone().url.eval(&variables)?;
        let mut url = match eval_url(url) {
            Err(e) => return Err(Error { source_info: self.clone().url.source_info, inner: e, assert: false }),
            Ok(url) => url
        };
        //let mut url =  format!("{}?", url);


        if !self.clone().querystring_params().is_empty() {
            let mut querystring_params = vec![];
            for param in self.clone().querystring_params() {
                let name = param.name.value;
                let value = param.value.eval(variables)?;
                querystring_params.push(format!("{}={}", name, value));
            }

            url.querystring = Some(querystring_params.join("&"));
        }


        // headers
        let user_agent = format!("hurl/{}", clap::crate_version!());
        let default_headers = vec![
            (String::from("User-Agent"), user_agent.clone()),
            (String::from("Host"), String::from(url.clone().host))
        ];
        let mut headers: Vec<http::Header> = vec![];
        for header in self.clone().headers {
            headers.push(header.eval(variables)?);
        }


        // add default headers if not present
        for (name, value) in default_headers {
            if !has_header(&headers, name.clone()) {
                headers.push(http::Header { name, value });
            }
        }

        // add cookies
        let mut cookies = current_cookies.clone();
        // TODO cookie from header
        for cookie in self.clone().cookies() {
            cookies.insert(cookie.clone().name.value, http::Cookie {
                name: cookie.clone().name.value,
                value: cookie.clone().value.value,
            });
        }
        if !cookies.is_empty() {
            headers.push(http::Header::from_cookies(cookies.values().map(|c| c.clone()).collect()));
        }


        if !self.clone().form_params().is_empty() {
            headers.push(http::Header {
                name: String::from("Content-Type"),
                value: String::from("application/x-www-form-urlencoded"),
            });
        }

//        vec![
//            http::Header {
//                name: String::from("User-Agent"),
//                value: user_agent
//            }
//        ],


//        let querystring_params= vec![];

        let bytes = match self.clone().body {
            Some(body) => body.eval(context_dir)?,
            None => {
                if !self.clone().form_params().is_empty() {
                    let mut params = vec![];
                    for param in self.clone().form_params() {
                        let name = param.name.value;
                        let value = param.value.eval(variables)?;
                        params.push(http::Param {
                            name,
                            value,
                        });
                    }

                    http::encode_form_params(params)
                } else {
                    vec![]
                }
            }
        };

        let request = http::Request {
            method: self.method.eval(),
            url,
            headers,
            body: bytes,
        };
        return Ok(request);
    }
}
// endregion


impl Header {
    pub fn eval(self, variables: &HashMap<String, String>) -> Result<http::Header, Error> {
        let name = self.name.value;
        let value = self.value.eval(variables)?;
        return Ok(http::Header { name, value });
    }
}

// region method
impl Method {
    fn eval(self) -> http::Method {
        return match self {
            Method::Get => http::Method::Get,
            Method::Head => http::Method::Head,
            Method::Post => http::Method::Post,
            Method::Put => http::Method::Put,
            Method::Delete => http::Method::Delete,
            Method::Connect => http::Method::Connect,
            Method::Options => http::Method::Options,
            Method::Trace => http::Method::Trace,
        };
    }
}
// endregion

#[cfg(test)]
pub fn hello_request() -> Request {

    // GET {{base_url}}/hello
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let line_terminator = LineTerminator {
        space0: whitespace.clone(),
        comment: None,
        newline: whitespace.clone(),
    };
    Request {
        line_terminators: vec![],
        space0: whitespace.clone(),
        method: Method::Get,
        space1: whitespace.clone(),
        url: HurlTemplate {
            elements: vec![
                HurlTemplateElement::Expression {
                    value: Expr {
                        space0: whitespace.clone(),
                        variable: Variable {
                            name: String::from("base_url"),
                            source_info: SourceInfo::init(1, 7, 1, 15),
                        },
                        space1: whitespace.clone(),
                    }
                },
                HurlTemplateElement::Literal { value: HurlString2 { value: String::from("/hello"), encoded: None } }
            ],
            delimiter: String::from(""),
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        line_terminator0: line_terminator.clone(),
        headers: vec![],
        sections: vec![],
        body: None,
        source_info: SourceInfo::init(0, 0, 0, 0),
    }
}

#[cfg(test)]
pub fn param(name: String, value: HurlTemplate) -> Param {
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let line_terminator = LineTerminator {
        space0: whitespace.clone(),
        comment: None,
        newline: whitespace.clone(),
    };
    Param {
        line_terminators: vec![],
        space0: whitespace.clone(),
        name: HurlString {
            value: name.clone(),
            encoded: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        space1: whitespace.clone(),
        space2: whitespace.clone(),
        value,
        line_terminator0: line_terminator.clone(),
    }
}

#[cfg(test)]
pub fn query_request() -> Request {
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };
    let line_terminator = LineTerminator {
        space0: whitespace.clone(),
        comment: None,
        newline: whitespace.clone(),
    };
    Request {
        line_terminators: vec![],
        space0: whitespace.clone(),
        method: Method::Get,
        space1: whitespace.clone(),
        url: HurlTemplate {
            elements: vec![
                HurlTemplateElement::Literal { value: HurlString2 { value: String::from("http://localhost:8000/querystring-params"), encoded: None } }
            ],
            delimiter: String::from(""),
            source_info: SourceInfo::init(0, 0, 0, 0),
        },
        line_terminator0: line_terminator.clone(),
        headers: vec![],
        sections: vec![
            Section {
                line_terminators: vec![],
                space0: whitespace.clone(),
                line_terminator0: line_terminator.clone(),
                value: SectionValue::QueryParams(vec![
                    param(String::from("param1"), HurlTemplate {
                        elements: vec![
                            HurlTemplateElement::Expression {
                                value: Expr {
                                    space0: whitespace.clone(),
                                    variable: Variable {
                                        name: String::from("param1"),
                                        source_info: SourceInfo::init(1, 7, 1, 15),
                                    },
                                    space1: whitespace.clone(),
                                }
                            },
                        ],
                        delimiter: "".to_string(),
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    }),
                    param(String::from("param2"), HurlTemplate {
                        elements: vec![],
                        delimiter: "".to_string(),
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    })
                ]),
            },
        ],
        body: None,
        source_info: SourceInfo::init(0, 0, 0, 0),
    }
}

#[test]
pub fn test_error_variable() {
    let variables = HashMap::new();
    let cookies = HashMap::new();
    let error = hello_request().eval(&variables, &cookies, "current_dir").err().unwrap();
    assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 15));
    assert_eq!(error.inner, RunnerError::TemplateVariableNotDefined { name: String::from("base_url") });
}

#[test]
pub fn test_hello_request() {
    let mut variables = HashMap::new();
    let cookies = HashMap::new();
    variables.insert(String::from("base_url"), String::from("http://localhost:8000"));
    assert_eq!(hello_request().eval(&variables, &cookies, "current_dir").unwrap(), http::hello_http_request());
}

#[test]
pub fn test_query_request() {
    let mut variables = HashMap::new();
    let cookies = HashMap::new();
    variables.insert(String::from("param1"), String::from("value1"));
    assert_eq!(query_request().eval(&variables, &cookies, "current_dir").unwrap(), http::query_http_request());
}


pub fn split_url(url: String) -> (String, Vec<http::Param>) {
    return match url.find('?') {
        None => (url, vec![]),
        Some(index) => {
            let (url, params) = url.split_at(index);
            //println!("params={:?}", params);
            let params: Vec<http::Param> = params[1..].split('&')
                .map(|s| {
                    return match s.find('=') {
                        None => http::Param { name: s.to_string(), value: String::from("") },
                        Some(index) => {
                            let (name, value) = s.split_at(index);
                            return http::Param { name: name.to_string(), value: value[1..].to_string() };
                        }
                    };
                })
                .collect();

            (url.to_string(), params)
        }
    };
}

#[test]
pub fn test_split_url() {
    assert_eq!(
        split_url(String::from("http://localhost:8000/hello")),
        (String::from("http://localhost:8000/hello"), vec![])
    );
    assert_eq!(
        split_url(String::from("http://localhost:8000/hello?param1=value1")),
        (String::from("http://localhost:8000/hello"), vec![http::Param { name: String::from("param1"), value: String::from("value1") }])
    );
}

// region url


pub fn eval_url(s: String) -> Result<http::Url, RunnerError> {
    return match url::Url::parse(s.as_str()) {
        Err(_) => Err(RunnerError::InvalidURL(s)),
        Ok(u) => Ok(http::Url {
            scheme: u.scheme().to_string(),
            host: u.host_str().unwrap().to_string(),
            port: u.port(),
            path: u.path().to_string(),
            querystring: match u.query() {
                None => None,
                Some(s) => Some(s.to_string())
            },
        })
    };
}

#[test]
pub fn test_eval_url() {
    assert_eq!(eval_url(String::from("xxx")).err().unwrap(), RunnerError::InvalidURL(String::from("xxx")));

    let url = eval_url(String::from("http://localhost:8000/querystring-params?param1=value1")).unwrap();
    assert_eq!(url.host, "localhost");
    assert_eq!(url.port, Some(8000));
    assert_eq!(url.querystring.unwrap(), String::from("param1=value1"));
}

// endregion

