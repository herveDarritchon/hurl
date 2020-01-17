use cookie::Cookie as ExternalCookie;

use super::core::*;
//use std::collections::HashMap;

// cookies
// keep cookies same name different domains
// send the most specific?? send the 2 of them?
// more flexible to keep list of cookies internally


pub type Domain = String;
pub type Name = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub max_age: Option<i64>,
    pub domain: Option<String>,
    pub path: Option<String>,
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
        let path = match c.path() {
            None => None,
            Some(v) => Some(v.to_string())
        };
        return Cookie { name, value, max_age, domain, path };
    }

    pub fn to_string(&self) -> String {
        let max_age = match self.clone().max_age {
            None => String::from(""),
            Some(v) => format!("; Max-Age:{}", v)
        };
        let domain = match self.clone().domain {
            None => String::from(""),
            Some(v) => format!("; Domain:{}", v)
        };
        let path = match self.clone().path {
            None => String::from(""),
            Some(v) => format!("; Path:{}", v)
        };
        return format!("{}={}{}{}{}",
                       self.name,
                       self.value,
                       max_age,
                       domain,
                       path
        );
    }

//    pub fn to_header(&self) -> Header {
//        return Header {
//            name: String::from("Cookie"),
//            value: format!("{}={}", self.name, self.value),
//        };
//        //format!("Cookie: {}", self.to_string());
//    }


    pub fn encode_cookie(header_name: String, header_value: String) -> Header {
        let name = String::from("Cookie");
        let value = format!("{}={};", header_name, header_value);
        return Header { name, value };
    }
}


//#[derive(Clone, Debug, PartialEq)]
//pub struct CookieStore {
//    inner: HashMap<Domain, Vec<Cookie>>
//}


#[derive(Clone, Debug, PartialEq)]
pub struct CookieJar {
    inner: Vec<InternalCookie>
}
impl CookieJar {

    pub fn init() -> CookieJar {
        return CookieJar { inner: vec![]};
    }

    pub fn cookies(self) -> Vec<Cookie> {
        return self.inner
            .iter()
            .map(|c| Cookie {
                name: c.clone().name,
                value: c.clone().value,
                max_age: None,
                domain: Some(c.domain.clone()),
                path: Some(c.path.clone())
            })
            .collect();
    }


    pub fn get_cookies(self, domain: String, path:String) -> Vec<Cookie> {
        return self.inner
            .iter()
            .filter(|c| c.is_usable(domain.clone(), path.clone()))
            .map(|c| Cookie {
                name: c.clone().name,
                value: c.clone().value,
                max_age: None,
                domain: Some(c.domain.clone()),
                path: Some(c.path.clone())
            })
            .collect();
    }

    pub fn update_cookies(&mut self, default_domain: String, _default_path:String, cookie: Cookie) {

          match cookie.max_age {
              Some(0) => {
                  //eprintln!("delete cookie {:?}", cookie);
                  self.inner.retain(|c| c.name != cookie.name);
              },
              _ => {

                  // replace value if same name+domain
                  let domain = match cookie.clone().domain {
                      None => default_domain,
                      Some(d) => d,
                  };
                  let path = match cookie.clone().path {
                      None => String::from("/"), // do not use default path for the time-beingdefault_path,
                      Some(p) => p,
                  };

                  // find existing cookie
                  for c in self.inner.iter_mut() {
                      if c.name == cookie.name && c.domain == domain {
                          c.value = cookie.value;
                          return;
                      }
                  }

                  // push new cookie
                  self.inner.push(InternalCookie {
                      name: cookie.clone().name,
                      value: cookie.clone().value,
                      domain,
                      path,
                      subdomains: !cookie.domain.is_none()
                  });
              }
          }

    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
struct InternalCookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub subdomains: bool
}

impl InternalCookie {
    fn is_usable(&self, domain: String, path: String) -> bool {

        // domain
        if !is_subdomain(self.clone().domain, domain.clone(), ) {
            return false;
        }
        if !self.subdomains && domain != self.clone().domain {
            return false;
        }

        // path
        if !is_subpath(self.clone().path, path, ) {
            return false;
        }
        return true;
    }


}


#[cfg(test)]
fn cookie_lsid() -> InternalCookie {
    return InternalCookie {
        name: String::from("LSID"),
        value: String::from("DQAAAK…Eaem_vYg"),
        domain: String::from("docs.foo.com"),
        path: String::from("/accounts"),
        subdomains: false
    };
}

#[cfg(test)]
fn cookie_hsid() -> InternalCookie {
   return   InternalCookie {
       name: String::from("HSID"),
       value: String::from("AYQEVn…DKrdst"),
       domain: String::from(".foo.com"),
       path: String::from("/"),
       subdomains: true
   };
}

#[cfg(test)]
fn cookie_ssid() -> InternalCookie {
    return InternalCookie {
        name: String::from("SSID"),
        value: String::from("Ap4P…GTEq"),
        domain: String::from("foo.com"),
        path: String::from("/"),
        subdomains: true
    };
}

#[cfg(test)]
fn sample_cookiejar() -> CookieJar {
    return CookieJar {
        inner: vec![
            cookie_lsid(),
            cookie_hsid(),
            cookie_ssid(),
        ]
    };
}


// question
// domain split with .
// what about path? split on / ???

#[test]
fn test_is_usable() {


    let domain = String::from("example.org");
    let path = String::from("/");
    assert_eq!(cookie_lsid().is_usable(domain.clone(), path.clone()), false);
    assert_eq!(cookie_hsid().is_usable(domain.clone(), path.clone()), false);
    assert_eq!(cookie_ssid().is_usable(domain.clone(), path.clone()), false);

    let domain = String::from("foo.com");
    let path = String::from("/");
    assert_eq!(cookie_lsid().is_usable(domain.clone(), path.clone()), false);
    assert_eq!(cookie_hsid().is_usable(domain.clone(), path.clone()), true);
    assert_eq!(cookie_ssid().is_usable(domain.clone(), path.clone()), true);

    let domain = String::from("foo.com");
    let path = String::from("/accounts");
    assert_eq!(cookie_lsid().is_usable(domain.clone(), path.clone()), false);
    assert_eq!(cookie_hsid().is_usable(domain.clone(), path.clone()), true);
    assert_eq!(cookie_ssid().is_usable(domain.clone(), path.clone()), true);

    let domain = String::from("docs.foo.com");
    let path = String::from("/accounts");
    assert_eq!(cookie_lsid().is_usable(domain.clone(), path.clone()), true);
    assert_eq!(cookie_hsid().is_usable(domain.clone(), path.clone()), true);
    assert_eq!(cookie_ssid().is_usable(domain.clone(), path.clone()), true);

}


#[test]
fn test_get_cookies() {

    let domain = String::from("docs.foo.com");
    let path = String::from("/accounts");
    assert_eq!(sample_cookiejar().get_cookies(domain, path).len(), 3);

    let domain = String::from("toto.docs.foo.com");
    let path = String::from("/accounts");
    assert_eq!(sample_cookiejar().get_cookies(domain, path).len(), 2);


}


// region domain

fn is_subdomain(domain: String, subdomain: String) -> bool {
    if domain == String::from("") {
        return false;
    }

    let mut domain_segments : Vec<&str> = domain.split(".").collect();
    if domain_segments.get(0).unwrap().clone() == "" {
        domain_segments.remove(0);
    }
    domain_segments.reverse();

    let mut subdomain_segments : Vec<&str> = subdomain.split(".").collect();
    if subdomain_segments.get(0).unwrap().clone() == "" {
        subdomain_segments.remove(0);
    }
    subdomain_segments.reverse();
    if domain_segments.len() > subdomain_segments.len() {
        return false;
    }

    for i in 0..domain_segments.len() {
        if domain_segments.get(i).unwrap() != subdomain_segments.get(i).unwrap() {
            return false;
        }
    }

    return true;
}

#[test]
fn test_is_subdomain() {

    assert_eq!(is_subdomain(String::from("foo.example.org"), String::from("example.org")), false);
    assert_eq!(is_subdomain(String::from("example.org"), String::from("toto.org")), false);

    assert_eq!(is_subdomain(String::from("example.org"), String::from("example.org")), true);
    assert_eq!(is_subdomain(String::from("example.org"), String::from("foo.example.org")), true);
    assert_eq!(is_subdomain(String::from(".example.org"), String::from("foo.example.org")), true);
}

// endregion

// region path

fn is_subpath(path: String, subpath: String) -> bool {
    if path == String::from("") {
        return false;
    }

    let mut path_segments : Vec<&str> = path.split("/").collect();
    if path_segments.get(0).unwrap().clone() == "" {
        path_segments.remove(0);
    }
    path_segments.reverse();
    if path_segments.get(0).unwrap().clone() == "" {
        path_segments.remove(0);
    }

    let mut subpath_segments : Vec<&str> = subpath.split("/").collect();
    if subpath_segments.get(0).unwrap().clone() == "" {
        subpath_segments.remove(0);
    }
    subpath_segments.reverse();
    if path_segments.len() > subpath_segments.len() {
        return false;
    }


    for i in 0..path_segments.len() {
        if path_segments.get(i).unwrap() != subpath_segments.get(i).unwrap() {
            return false;
        }
    }

    return true;
}



#[test]
fn test_is_subpath() {
    assert_eq!(is_subpath(String::from("/toto"), String::from("/toto")), true);
    assert_eq!(is_subpath(String::from("/"), String::from("/toto")), true);
    assert_eq!(is_subpath(String::from("/to"), String::from("/toto")), false);

}

// endregion

//impl CookieStore {
//    pub fn init() -> CookieStore {
//        return CookieStore { inner: HashMap::new() };
//    }
//
//    // TODO - add check
//    // TODO - add delete with Max-Age
//   pub fn update(&mut self, domain: Domain, cookie: Cookie) {
////        let domain = match cookie.domain {
////            None => url.host,
////            Some(v) => if is_sub_domain(url.host, v.clone()) {
////                v
////            } else {
////                return;
////            }
////        };
////
////        self.cookies.push(Cookie {
////            name: cookie.name,
////            value: cookie.value,
////            max_age: cookie.max_age,
////            domain: Some(domain),
////        });
//        let mut domain_cookies = match self.inner.get(domain.as_str()) {
//             None => {
//                vec![]
//             }
//             Some(v) => {
//                v.clone()
//             }
//         };
//        match cookie.max_age {
//            Some(0) => {
//                domain_cookies.retain(|c| c.name != cookie.name );
//            },
//            _ => {
//                domain_cookies.push(cookie);
//            }
//        }
//
//        self.inner.insert(domain, domain_cookies);
//
//    }
//
//    // TODO - add check
//    pub fn get_cookies(self, domain: Domain) -> Vec<Cookie> {
//        return match self.inner.get(domain.as_str()) {
//            None => vec![],
//            Some(v)=> v.clone()
//        };
//    }
//}


