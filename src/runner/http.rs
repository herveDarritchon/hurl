extern crate reqwest;
use serde::{Deserialize, Serialize};

use super::core::RunnerError;


pub struct Client {
    inner_client: reqwest::Client,
    _options: ClientOptions,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientOptions {
    pub noproxy_hosts: Vec<String>,
    pub insecure: bool,
}

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
    fn to_reqwest(self) -> reqwest::Method {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub method: Method,
    pub url: Url,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub querystring: Option<String>,
}

impl Url {
    pub fn to_string(&self) -> String {
        let port = match self.port {
            None => String::from(""),
            Some(p) => format!(":{}", p),
        };
        let mut s = format!("{}://{}{}{}", self.scheme, self.host, port, self.path);
        match self.clone().querystring {
            None => {}
            Some(q) => s.push_str(format!("?{}", q).as_str())
        }
        return s;
    }
}

#[test]
fn test_to_string() {
    assert_eq!(hello_url().to_string(), String::from("http://localhost:8000/hello"))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
}

impl Cookie {
    pub fn to_string(&self) -> String {
        return format!("{}={}", self.name, self.value);
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    pub version: Version,
    pub status: u16,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Version {
    Http10,
    Http11,
    Http2,
}

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
    s.push_str(format!("{}={}", param.name, param.value).as_str());
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
    assert_eq!(
        url,
        String::from("http://localhost:5000/querystring-params?param1=value1&param2=")
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


pub fn encode_cookie(header_name: String, header_value: String) -> Header {
    let name = String::from("Cookie");
    let value = format!("{}={};", header_name, header_value);
    return Header { name, value };
}


impl Response {
    pub fn get_header(&self, name: &str, case_sensitive: bool) -> Option<String> {
        for header in self.headers.clone() {
            if header.name == name
                || !case_sensitive && header.name.to_lowercase() == name.to_lowercase()
            {
                return Some(header.value);
            }
        }
        return None;
    }

    pub fn get_cookie(&self, name: &str) -> Option<String> {
        for cookie in self.cookies() {
            if cookie.name == name.to_string()
            {
                return Some(cookie.value);
            }
        }
        return None;
    }

    pub fn cookies(&self) -> Vec<Cookie> {
        let mut cookies = vec![];
        for Header { name, value } in self.clone().headers {
            if name.to_lowercase() == "set-cookie" {
                let fields = value.split(";").collect::<Vec<&str>>();
                let name_value = fields.get(0).unwrap().split("=").collect::<Vec<&str>>();
                let name = name_value.get(0).unwrap().to_string();
                let value = name_value.get(1).unwrap().to_string();
                cookies.push(Cookie { name, value });
            }
        }
        return cookies;
    }
}


impl Client {
    pub fn init(options: ClientOptions) -> Client {

//let mut headers = reqwest::header::HeaderMap::new();
//let user_agent = format!("hurl/{}",clap::crate_version!());
//headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_str(user_agent.as_str()).unwrap());
//headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static(""));
//eprintln!("{}", clap::crate_version!());

        let client_builder = reqwest::Client::builder()
            .redirect(reqwest::RedirectPolicy::none())
//.default_headers(headers)
            .use_sys_proxy()
            .danger_accept_invalid_hostnames(options.insecure)
            .danger_accept_invalid_certs(options.insecure)
//.http1_title_case_headers()
            .cookie_store(true);
        return Client {
            inner_client: client_builder.build().unwrap(),
            _options: options,
        };
    }

    //pub fn execute(&self, request: &Request) -> Result<Response, Error> {
    pub fn execute(&self, request: &Request) -> Result<Response, RunnerError> {
        let mut headers = reqwest::header::HeaderMap::new();
        for header in request.clone().headers {
            headers.insert(
                reqwest::header::HeaderName::from_lowercase(
                    header.name.to_lowercase().as_str().as_bytes(),
                )
                    .unwrap(),
                reqwest::header::HeaderValue::from_str(header.value.as_str()).unwrap(),
            );
        }


//        let url = if request.querystring_params.is_empty() {
//            request.clone().url
//        } else {
//            let params: Vec<String> = request.querystring_params
//                .iter()
//                .map(|p| format!("{}={}", p.name, p.value))
//                .collect();
//            format!("{}?{}", request.clone().url, params.join("&"))
//        };


        // no proxy variable depends on the request
        // rebuilding the client => you will need to keep the cookie yourself
//        let client_builder = reqwest::Client::builder()
//            .redirect(reqwest::RedirectPolicy::none())
//            .use_sys_proxy()
//            .cookie_store(true);
//
//        let client_builder = if self.options.noproxy_hosts.contains(&request.url.host.clone()) {
//            client_builder.no_proxy()
//        } else {
//            client_builder
//        };
//        let client = client_builder.build().unwrap();
        let client = &self.clone().inner_client;

        let req = client
            .request(
                request.clone().method.to_reqwest(),
                reqwest::Url::parse(request.url.to_string().as_str()).unwrap(),
            )
            .headers(headers)
            .body(request.clone().body)
            .build()
            .unwrap();


        match client
            .execute(req) {
            Ok(mut resp) => {
                let mut headers = vec![];
                for (name, value) in resp.headers() {
                    headers.push(Header {
                        name: name.as_str().to_string(),
                        value: value.to_str().unwrap().to_string(),
                    })
                }

                let version = match resp.version() {
                    reqwest::Version::HTTP_10 => Version::Http10,
                    reqwest::Version::HTTP_11 => Version::Http11,
                    reqwest::Version::HTTP_2 => Version::Http2,
                    v => panic!("Version {:?} not supported!", v),
                };
                let mut buf: Vec<u8> = vec![];
                resp.copy_to(&mut buf).unwrap();
                resp.content_length(); // dirty hack to prevent error "connection closed before message completed"?


                // extract cookies
                //eprintln!("[DEBUG] Cookies");
                //for cookie in resp.cookies() {
                //eprintln!("[DEBUG]  {}", cookie.name());
                //}
                return Ok(Response {
                    version,
                    status: resp.status().as_u16(),
                    headers,
                    body: buf,
                });
            }
            Err(e) => {
                return Err(RunnerError::HttpConnection {
                    message: format!("{:?}", e.to_string()),
                    url: request.url.to_string(),
                });
            }
        }
    }
}

impl Header {
    pub fn from_cookies(cookies: Vec<Cookie>) -> Header {
        return Header {
            name: String::from("Cookie"),
            value: cookies
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("; "),
        };
    }
}


impl Response {
    pub fn has_utf8_body(&self) -> bool {
        return match self.get_header("content-type", false) {
            Some(s) => {
                if s.contains("charset=utf-8") {
                    true
                } else {
                    false
                }
            }
            _ => false
        };
    }


    pub fn is_html(&self) -> bool {
        return match self.get_header("content-type", false) {
            Some(s) => {
                if s.contains("html") {
                    true
                } else {
                    false
                }
            }
            _ => false
        };
    }
}

#[test]
fn test_cookie_header() {
    assert_eq!(
        Header::from_cookies(vec![]),
        Header {
            name: String::from("Cookie"),
            value: String::from(""),
        }
    );
    assert_eq!(
        Header::from_cookies(vec![Cookie {
            name: String::from("cookie1"),
            value: String::from("value1"),
        }]),
        Header {
            name: String::from("Cookie"),
            value: String::from("cookie1=value1"),
        }
    );
    assert_eq!(
        Header::from_cookies(vec![
            Cookie { name: String::from("cookie1"), value: String::from("value1") },
            Cookie { name: String::from("cookie2"), value: String::from("value2") }
        ]),
        Header {
            name: String::from("Cookie"),
            value: String::from("cookie1=value1; cookie2=value2"),
        }
    );
}


pub fn get_header_value(headers: Vec<Header>, name: &str) -> Option<String> {
    for header in headers {
        if header.name == name.to_string() {
            return Some(header.value.clone());
        }
    }
    return None;
}

#[cfg(test)]
pub fn hello_url() -> Url {
    Url {
        scheme: String::from("http"),
        host: String::from("localhost"),
        port: Some(8000),
        path: String::from("/hello"),
        querystring: None,
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


#[cfg(test)]
pub fn hello_http_response() -> Response {
    return Response {
        version: Version::Http10,
        status: 200,
        headers: vec![
            Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
            Header { name: String::from("Content-Length"), value: String::from("12") },
        ],
        body: String::into_bytes(String::from("Hello World!")),
    };
}

#[cfg(test)]
pub fn html_http_response() -> Response {
    return Response {
        version: Version::Http10,
        status: 200,
        headers: vec![
            Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
        ],
        body: String::into_bytes(String::from("<html><head><meta charset=\"UTF-8\"></head><body><br></body></html>")),
    };
}


#[cfg(test)]
pub fn xml_invalid_response() -> Response {
    return Response {
        version: Version::Http10,
        status: 200,
        headers: vec![
            Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
            Header { name: String::from("Content-Length"), value: String::from("12") },
        ],
        body: String::into_bytes(r#"
xxx
"#.to_string()),
    };
}

#[cfg(test)]
pub fn xml_two_users_http_response() -> Response {
    return Response {
        version: Version::Http10,
        status: 200,
        headers: vec![
            Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
            Header { name: String::from("Content-Length"), value: String::from("12") },
        ],
        body: String::into_bytes(r#"
<?xml version="1.0"?>
<users>
  <user id="1">Bob</user>
  <user id="2">Bill</user>
</users>
"#.to_string()),
    };
}

#[cfg(test)]
pub fn xml_three_users_http_response() -> Response {
    return Response {
        version: Version::Http10,
        status: 200,
        headers: vec![
            Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
            Header { name: String::from("Content-Length"), value: String::from("12") },
        ],
        body: String::into_bytes(r#"
<?xml version="1.0"?>
<users>
  <user id="1">Bob</user>
  <user id="2">Bill</user>
  <user id="3">Bruce</user>
</users>
"#.to_string()),
    };
}

#[cfg(test)]
pub fn json_http_response() -> Response {
    return Response {
        version: Version::Http10,
        status: 0,
        headers: vec![],
        body: String::into_bytes(r#"
{
  "success":false,
  "errors": [
    { "id": "error1"},
    {"id": "error2"}
  ]
}
"#.to_string()),
    };
}