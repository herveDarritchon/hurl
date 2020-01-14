use cookie::Cookie as ExternalCookie;
use super::core::*;
use serde::{Deserialize, Serialize};


pub type Domain = String;
pub type Name = String;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub max_age: Option<i64>,
    pub domain: Option<String>,
}

impl Cookie {
    pub fn from_str(s: &str) -> Cookie {
        let c = ExternalCookie::parse(s).unwrap();
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
        return Cookie { name, value, max_age, domain };
    }

    pub fn to_string(&self) -> String {
        let max_age = match self.max_age {
            None => String::from(""),
            Some(v) => format!(";Max-Age:{}", v)
        };
        return format!("{}={}{}", self.name, self.value, max_age);
    }
    pub fn to_header(&self) -> Header {
        return Header {
            name: String::from("Cookie"),
            value: self.to_string()
        };
        //format!("Cookie: {}", self.to_string());
    }


    pub fn encode_cookie(header_name: String, header_value: String) -> Header {
        let name = String::from("Cookie");
        let value = format!("{}={};", header_name, header_value);
        return Header { name, value };
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CookieStore {
    cookies: Vec<Cookie>
}

impl CookieStore {
    fn init() -> CookieStore {
        return CookieStore { cookies: vec![] };
    }
    fn add(&mut self, url: Url, cookie: Cookie) {
        self.cookies.push(cookie);
    }

    fn get_cookies(self, url: Url) -> Vec<Cookie> {
        return self.cookies;
    }
}


#[test]
fn test_cookie_store() {
    let mut cookie_jar = CookieStore::init();
    let url = Url::eval(String::from("http://localhost:8000/hello")).unwrap();

    cookie_jar.add(url, Cookie::from_str("cookie1=value1;"));
    assert_eq!(cookie_jar.get_cookies(Url::eval(String::from("http://localhost:8000/hello")).unwrap()).len(), 1);
}




//
//#[test]
//fn test_cookie_header() {
//    assert_eq!(
//        Header::from_cookies(vec![]),
//        Header {
//            name: String::from("Cookie"),
//            value: String::from(""),
//        }
//    );
//    assert_eq!(
//        Header::from_cookies(vec![Cookie {
//            name: String::from("cookie1"),
//            value: String::from("value1"),
//            max_age: None,
//            domain: None,
//        }]),
//        Header {
//            name: String::from("Cookie"),
//            value: String::from("cookie1=value1"),
//        }
//    );
//    assert_eq!(
//        Header::from_cookies(vec![
//            Cookie { name: String::from("cookie1"), value: String::from("value1"), max_age: None, domain: None },
//            Cookie { name: String::from("cookie2"), value: String::from("value2"), max_age: None, domain: None },
//        ]),
//        Header {
//            name: String::from("Cookie"),
//            value: String::from("cookie1=value1; cookie2=value2"),
//        }
//    );
//}