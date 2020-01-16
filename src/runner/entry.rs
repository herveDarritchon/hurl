use std::collections::HashMap;

use crate::core::ast::*;
use crate::core::core::SourceInfo;
//use crate::core::core::Value;
use crate::http;

use super::core::*;
use super::core::{Error, RunnerError};
use super::text::*;
use crate::http::cookie::CookieStore;


// cookies
// for all domains

// but only pass cookies for one domain for the request


impl Entry {
    pub fn eval(self, http_client: &http::client::Client,
                variables: &mut HashMap<String, String>,
                cookie_store: &mut CookieStore,
                verbose: bool,
                context_dir: &str,
    ) -> EntryResult {

        //let mut entry_log_builder = EntryLogBuilder::init();

        let mut http_request = match self.clone().request.eval(variables, context_dir) {
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
        http_request.add_session_cookies(cookie_store.clone().get_cookies(http_request.clone().host()));

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
                            inner: RunnerError::HttpConnection {
                                message: e.message,
                                url: e.url,
                            },
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
            Some(response) => response.eval_asserts(variables, http_response.clone())
        };

        let errors = asserts
            .iter()
            .filter_map(|assert| assert.clone().error())
            .map(|Error { source_info, inner, .. }| Error { source_info, inner, assert: true })
            .collect();


        // update cookies
        // for the domain
        let domain = http_request.clone().host();
        //let mut cookies = cookie_store.get_cookies(host);


        for cookie in http_response.cookies() {

//            if verbose {
//                let max_age = match cookie.max_age {
//                    Some(value) => format!(";Max-Age={}", value),
//                    None => String::from("")
//                };
//                //eprintln!("[DEBUG] cookie {}={}{}", cookie.name, cookie.value, max_age);
//            }
            cookie_store.update(domain.clone(), cookie);
//            match cookie.max_age {
//                Some(0) => {
//
//                   // cookies.remove(cookie.clone().name.as_str());
//                    //eprintln!(">>> cookies={:?}", cookies);
//                }
//                _ => {
//                    //cookies.insert(cookie.clone().name, cookie);
//                }
//            }
        }
        if verbose {
            eprintln!("[DEBUG] Cookies for {}", domain);
            for cookie in cookie_store.clone().get_cookies(domain) {
                eprintln!("[DEBUG] {}", cookie.to_string());
            }
        }



        return EntryResult {
            request: Some(http_request),
            response: Some(http_response),
            captures,
            asserts,
            errors,
        };
    }
}
