use super::http::*;

pub trait Textable {
    fn to_text(&self) -> String;
}


impl Textable for Request {
    fn to_text(&self) -> String {
        let mut s = format!("{} {}\n", self.method.to_text(), self.url.to_string());
        for header in self.headers.clone() {
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

        let limit_body = 100; // TODO should be explicitly pass as a command-line argument
        let body = body_text(self.clone().body, get_header_value(self.clone().headers, "content-type"));
        let body = if body.len() < limit_body {
            body
        } else {
            format!("{}...", &body[0..limit_body])
        };
        s.push_str(body.as_str());
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
    match content_type {
        Some(content_type) =>
            if vec!["application/json", "text/html;charset=utf-8", "text", "application/x-www-form-urlencoded", "text/html"].contains(&content_type.as_str()) {
                return String::from_utf8(bytes).unwrap();
            } else if &content_type == "text/html; charset=utf-8" {
                match String::from_utf8(bytes.clone()) {
                    Ok(s) => return s,
                    _ => {}
                };
            }
        _ => {}
    }
    return if bytes.is_empty() { String::from("") } else { format!("{:?}", bytes) };
}


