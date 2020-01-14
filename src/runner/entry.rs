use std::collections::HashMap;

use serde::Serialize;
//use serde::ser::SerializeStruct;

use crate::core::ast::*;
use crate::core::core::SourceInfo;
use crate::core::core::Value;

use super::assert::AssertResult;
use super::core::Error;
use crate::http;
use super::text::*;

//pub type EntryResult = Result<EntryLog, Error>;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EntryResult {
    pub request: Option<http::request::Request>,
    pub response: Option<http::response::Response>,
    pub captures: Vec<(String, Value)>,
    pub asserts: Vec<AssertResult>,
    pub errors: Vec<Error>,
}

//impl Serialize for EntryResult {
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//        where
//            S: Serializer,
//    {
//        // 3 is the number of fields in the struct.
//        let mut state = serializer.serialize_struct("Color", 3)?;
//        state.end()
//    }
//}


//#[derive(Clone, Debug, PartialEq, Eq,Serialize, Deserialize)]
//#[derive(Clone, Debug, PartialEq, Eq)]
//pub struct EntryLog {
//    pub request: http::Request,
//    pub response: http::Response,
//    pub captures: Vec<(String, Value)>,
//    pub asserts: Vec<AssertResult>,
//}


//impl EntryLog {
//    pub fn fail(self) -> bool {
//       !self.errors().is_empty()
//    }
//    pub fn success(self) -> bool {
//        self.errors().is_empty()
//    }
//
//
//    pub fn errors(self) -> Vec<Error> {
//        return self.asserts.iter().filter_map(|e| e.clone().error()).collect();
//    }
//}

//impl EntryResult {
//
//    pub fn errors(self) -> Vec<Error> {
//        return match self {
//            Ok(entry_log) => entry_log.errors(),
//            Err(e) => vec![e],
//        };
//    }
//}

// cookies
// for all domains

// but only pass cookies for one domain for the request



impl Entry {
    pub fn eval(self, http_client: &http::client::Client,
                variables: &mut HashMap<String, String>,
                all_cookies: &mut HashMap<http::cookie::Domain, HashMap<http::cookie::Name, http::cookie::Cookie>>,
                verbose: bool,
                context_dir: &str,
    ) -> EntryResult {

        //let mut entry_log_builder = EntryLogBuilder::init();

        let http_request = match self.clone().request.eval(variables, all_cookies, context_dir) {
            Ok(r) => r,
            Err(error) => {
                return EntryResult {
                    request: None,
                    response: None,
                    captures: vec![],
                    asserts: vec![],
                    errors: vec![error],
                };
            }
        };
        if verbose {
            eprintln!("---------------------------------------------------------------------------------------------------");
            eprintln!("{}", http_request.to_text())
        }


        let http_response = match http_client.execute(&http_request) {
            Ok(response) => response,
            Err(e) => {
                return EntryResult {
                    request: Some(http_request),
                    response: None,
                    captures: vec![],
                    asserts: vec![],
                    errors: vec![
                        Error {
                            source_info: SourceInfo {
                                start: self.clone().request.url.source_info.start,
                                end: self.clone().request.url.source_info.end,
                            },
                            inner: e,
                            assert: false,
                        }],
                };
            }
        };
        if verbose {
            eprintln!("{}", http_response.to_text())
        }
        //entry_log_builder.response(http_response.clone(), verbose);

        //hurl_log.entries.push(log_builder.build());
        let captures = match self.response.clone() {
            None => vec![],
            Some(response) => match response.eval_captures(variables, http_response.clone()) {
                Ok(captures) => captures,
                Err(e) => {
                    return EntryResult {
                        request: Some(http_request.clone()),
                        response: Some(http_response.clone()),
                        captures: vec![],
                        asserts: vec![],
                        errors: vec![e],
                    };
                }
            }
        };

        // update variables now!
        for (name, value) in captures.clone() {
            variables.insert(name, value.to_string());
        }


        let asserts = match self.response {
            None => vec![],
            Some(response) =>  response.eval_asserts(variables, http_response.clone())
        };

        let errors = asserts
            .iter()
            .filter_map(|assert| assert.clone().error())
            .map(|Error{ source_info, inner,..}| Error{ source_info, inner, assert:true})
            .collect();
//        if verbose {
//            for assert_result in asserts.clone() {
//                eprintln!("{:?} {}", assert_result.clone(), if assert_result.fail() { "Failure"} else { "Success"});
//            }
//        }

        // update cookies
        // for the domain
        let host = http_request.clone().host();
        let mut cookies: HashMap<http::cookie::Name, http::cookie::Cookie> = match all_cookies.get(&host) {
            None => HashMap::new(),
            Some(v) =>  v.clone(),
        };



        for cookie in http_response.cookies() {

//            if verbose {
//                let max_age = match cookie.max_age {
//                    Some(value) => format!(";Max-Age={}", value),
//                    None => String::from("")
//                };
//                //eprintln!("[DEBUG] cookie {}={}{}", cookie.name, cookie.value, max_age);
//            }

            match cookie.max_age {
                Some(0) => {

                   cookies.remove(cookie.clone().name.as_str());
                    //eprintln!(">>> cookies={:?}", cookies);
                }
                _ => {
                    cookies.insert(cookie.clone().name, cookie);
                }
            }

        }
        if verbose {
            eprintln!("[DEBUG] Cookies for {}", host);
            for (_, cookie) in cookies.clone() {
                eprintln!("[DEBUG] {}", cookie.to_string());
            }
        }

        all_cookies.insert(host, cookies.clone());

        return EntryResult {
            request: Some(http_request),
            response: Some(http_response),
            captures,
            asserts,
            errors,
        };
    }
}
