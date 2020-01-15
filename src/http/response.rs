use super::core::*;
use super::cookie::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub version: Version,
    pub status: u16,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Version {
    Http10,
    Http11,
    Http2,
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
                let c = cookie::Cookie::parse(value.as_str()).unwrap();
//                eprintln!(">>> parse set-cookie header");
//                eprintln!(">>> c = {:?}", c);
//
//                let fields = value.split(";").collect::<Vec<&str>>();
//                let name_value = fields.get(0).unwrap().split("=").collect::<Vec<&str>>();
//                let name = name_value.get(0).unwrap().to_string();
//                let value = name_value.get(1).unwrap().to_string();
                let name = c.name().to_string();
                let value = c.value().to_string();
                let max_age = match c.max_age() {
                    None => None,
                    Some(d) => Some(d.num_seconds())
                };
                let domain = match c.domain() {
                    None => None,
                    Some(v) => Some(v.to_string())
                };
                cookies.push(Cookie { name, value, max_age, domain });
            }
        }
        return cookies;
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


// region test

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
// endregion

