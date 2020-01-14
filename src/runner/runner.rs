use std::collections::HashMap;

use serde::Serialize;

use crate::core::ast::*;
use crate::core::core::FormatError;
use super::core::Error;
use super::entry::EntryResult;
//use crate::http::request::*;
//use crate::http::response::*;
use crate::http::client::*;
use super::super::format;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct HurlResult {
    pub filename: String,
    pub entries: Vec<EntryResult>,
}

impl HurlResult {
    pub fn errors(self) -> Vec<Error> {
        return self.entries.iter().flat_map(|e| e.errors.clone()).collect();
    }
}


pub fn run(
    http_client: Client,
    hurl_file: HurlFile,
    fail_fast: bool,
    init_variables: &HashMap<String, String>,
    verbose: bool,
    context_dir: &str,
    filename: String,
    output_color: bool,
    lines: Vec<String>,
) -> HurlResult {
    let mut entries = vec![];
    let mut variables = HashMap::new();
    let mut all_cookies = HashMap::new();
    for (key, value) in init_variables {
        variables.insert(key.to_string(), value.to_string());
    }

    //let mut variables = variables;
    for entry in hurl_file.entries {
        // eprintln!(">> entry");
        let entry_result = entry.eval(&http_client, &mut variables, &mut all_cookies, verbose, context_dir);
        entries.push(entry_result.clone());
        for e in entry_result.errors.clone() {
            let error = format::error::Error {
                exit_code: 3,
                source_info: e.clone().source_info,
                description: e.clone().description(),
                fixme: e.fixme(),
                lines: lines.clone(),
                filename: filename.to_string(),
                warning: false,
                color: output_color,
            };
            eprintln!("{}", error.format());
        }

        if fail_fast && !entry_result.errors.is_empty() {
            break;
        }
        // TODO TO BE CLARIFIED
//        let assert_errors: Vec<Error> = entry_result.errors.iter().filter(|e| e.assert).collect();
//        if !assert_errors.is_empty() {
//            break;
//        }


        //eprintln!(">>> all cookies {:#?}", all_cookies);

    }
    return HurlResult {
        filename,
        entries,
    };
}
