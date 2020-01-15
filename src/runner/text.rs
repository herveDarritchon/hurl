use crate::http::core::*;
use crate::http::request::*;
use crate::http::response::*;

pub trait Textable {
    fn to_text(&self) -> String;
}


impl Textable for Request {
    fn to_text(&self) -> String {
        let mut s = format!("{} {}\n",
                            self.clone().method.to_text(),
                            self.clone().url()
        );
        for header in self.clone().headers() {
            s.push_str(header.to_text().as_str());
        }
        s.push_str("\n");
        s.push_str(body_text(self.clone().body, get_header_value(self.clone().headers, "content-type")).as_str());
        return s;
    }
}


impl Textable for Response {
    fn to_text(&self) -> String {
        let mut s = format!("HTTP/{} {}\n", self.version.to_text(), self.status);
        for header in self.headers.clone() {
            s.push_str(header.to_text().as_str());
        }
        s.push_str("\n");

        // shoudl use number of char, not a number of bytes!!
        let limit_body = 200; // TODO should be explicitly pass as a command-line argument
        let body = body_text(self.clone().body, get_header_value(self.clone().headers, "content-type"));
//        let body = if body.len() < limit_body - 1 {
//            body
//        } else {
//            format!("{}...", &body[0..limit_body])
//        };
        s.push_str(substring(body.as_str(), 0, limit_body));
        return s;
    }
}

impl Textable for Method {
    fn to_text(&self) -> String {
        return match self {
            Method::Get => String::from("GET"),
            Method::Head => String::from("HEAD"),
            Method::Post => String::from("POST"),
            Method::Put => String::from("PUT"),
            Method::Delete => String::from("DELETE"),
            Method::Connect => String::from("CONNECT"),
            Method::Options => String::from("OPTIONS"),
            Method::Trace => String::from("TRACE"),
        };
    }
}

impl Textable for Version {
    fn to_text(&self) -> String {
        return match self {
            Version::Http10 => String::from("1.0"),
            Version::Http11 => String::from("1.1"),
            Version::Http2 => String::from("2"),
        };
    }
}

impl Textable for Header {
    fn to_text(&self) -> String {
        return format!("{}: {}\n", self.name, self.value);
    }
}


fn body_text(bytes: Vec<u8>, content_type: Option<String>) -> String {
    return match content_type {
        Some(content_type) =>
            if is_text(content_type.as_str()) {
                String::from_utf8(bytes).unwrap()
            } else {
                format!("{:?}", bytes)
            }
        _ => {
            if bytes.is_empty() {
                String::from("")
            } else {
                format!("{:?}", bytes)
            }
        }
    };
}

fn is_text(content_type: &str) -> bool {
    for s in vec![
        "application/json",
        "text/html",
        "charset=utf-8",
        "application/x-www-form-urlencoded"
    ] {
        if content_type.contains(s) {
            return true;
        }
    }
    return false;
}

#[test]
fn test_is_text() {
    assert_eq!(is_text("application/json"), true);
    assert_eq!(is_text("application/json;charset=utf-8"), true);
}


// substring implementation

fn substring(s: &str, start: usize, len: usize) -> &str {
    let mut char_pos = 0;
    let mut byte_start = 0;
    let mut it = s.chars();
    loop {
        if char_pos == start { break; }
        if let Some(c) = it.next() {
            char_pos += 1;
            byte_start += c.len_utf8();
        } else { break; }
    }
    char_pos = 0;
    let mut byte_end = byte_start;
    loop {
        if char_pos == len { break; }
        if let Some(c) = it.next() {
            char_pos += 1;
            byte_end += c.len_utf8();
        } else { break; }
    }
    &s[byte_start..byte_end]
}


#[test]
fn test_substring() {
    assert_eq!(substring("", 0, 0), "");
    assert_eq!(substring("hello world!", 0, 5), "hello");
    assert_eq!(substring("hello world!", 0, 15), "hello world!");
}