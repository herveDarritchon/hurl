extern crate hurl;

use hurl::core::ast;
use hurl::core::core::{Pos, SourceInfo};
use hurl::runner;
use hurl::http;
use std::collections::HashMap;
use std;
use hurl::core::ast::HurlString;


// can be used for debugging
#[test]
fn test_hurl_file() {
    let mut cookie_store = http::cookie::CookieJar::init();
    let filename = "integration/tests/cookies.hurl";
    //let filename = "integration/tests/error_assert_match_utf8.hurl";
    //let filename = "integration/tests/bytes.hurl";
    //let filename = "/mnt/secure/repos/work/myshop/integration/src/main/hurl-generated/pcm/pcm-jdd-open-up-150go.hurl";
    let content = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut parser = hurl::parser::core::Parser::init(content.as_str());
    let hurl_file = hurl::parser::parser::hurl_file(&mut parser).unwrap();
    let mut variables = HashMap::new();
    let client = http::client::Client::init(http::client::ClientOptions {
        noproxy_hosts: vec![],
        insecure: false,
    });
    let mut lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(&content)
        .collect();
    // edd an empty line at the end?
    lines.push("");
    let lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();

    let _hurl_log = runner::runner::run(
        client,
        hurl_file,
        true,
        &mut variables,
        true,
        "current_dir".to_string(),
        filename.to_string(),
        false,
        lines,
        &mut cookie_store
    );
//    assert_eq!(1,2)

}

#[cfg(test)]
fn hello_request() -> ast::Request {
    // GET http://localhost;8000/hello
    let source_info = SourceInfo {
        start: Pos { line: 1, column: 1 },
        end: Pos { line: 1, column: 1 },
    };
    let whitespace = ast::Whitespace {
        value: "".to_string(),
        source_info: source_info.clone(),
    };
    let line_terminator = ast::LineTerminator {
        space0: whitespace.clone(),
        comment: None,
        newline: whitespace.clone(),
    };
    ast::Request {
        line_terminators: vec![],
        space0: whitespace.clone(),
        method: ast::Method::Get,
        space1: whitespace.clone(),
        url: ast::HurlTemplate {
            elements: vec![
                ast::HurlTemplateElement::Literal {
                    value: ast::HurlString2 {
                        value: String::from("http://localhost:8000/hello"),
                        //value: String::from("http://xhttpbin.org/get"),
                        encoded: None,
                    }
                }
            ],
            delimiter: "".to_string(),
            source_info: SourceInfo::init(0, 0, 0, 0),
        }, //literal("http://localhost:8000/hello".to_string()),
        line_terminator0: ast::LineTerminator {
            space0: whitespace.clone(),
            comment: None,
            newline: whitespace.clone(),
        },
        headers: vec![
            ast::Header {
                line_terminators: vec![],
                space0: whitespace.clone(),
                name: HurlString {
                    value: String::from("User-Agent"),
                    encoded: None,
                    source_info: source_info.clone(),
                },
                space1: whitespace.clone(),
                space2: whitespace.clone(),
                value: ast::HurlTemplate {
                    elements: vec![
                        ast::HurlTemplateElement::Literal { value: ast::HurlString2 { value: String::from("test"), encoded: None } }
                    ],
                    delimiter: "".to_string(),
                    source_info: source_info.clone(),
                },
                line_terminator0: line_terminator.clone(),
            }
        ],
        sections: vec![],
        body: None,
        source_info: source_info.clone(),
    }
}

#[test]
fn test_hello() {
    let mut cookie_store = http::cookie::CookieJar::init();
    let client = http::client::Client::init(http::client::ClientOptions {
        noproxy_hosts: vec![],
        insecure: false,
    });
    let source_info = SourceInfo {
        start: Pos { line: 1, column: 1 },
        end: Pos { line: 1, column: 1 },
    };
    let whitespace = ast::Whitespace {
        value: "".to_string(),
        source_info: source_info.clone(),
    };
    let request = hello_request();
    let hurl_file = ast::HurlFile {
        entries: vec![ast::Entry {
            request,
            response: Some(ast::Response {
                line_terminators: vec![],
                version: ast::Version {
                    value: ast::VersionValue::Version11,
                    source_info: source_info.clone(),
                },
                space0: whitespace.clone(),
                status: ast::Status {
                    value: 200,
                    source_info: source_info.clone(),
                },
                space1: whitespace.clone(),
                line_terminator0: ast::LineTerminator {
                    space0: whitespace.clone(),
                    comment: None,
                    newline: whitespace.clone(),
                },
                headers: vec![],
                sections: vec![],
                body: None,
                source_info: source_info.clone(),
            }),
        }],
        line_terminators: vec![],
    };
    let lines = vec![
        String::from("line1")
    ];
    let mut variables = HashMap::new();
    let _hurl_log = runner::runner::run(
        client,
        hurl_file,
        false,
        &mut variables,
        true,
        "current_dir".to_string(),
        String::from("filename"),
        true,
        lines,
        &mut cookie_store
    );
    //assert_eq!(hurl_log.entries.len(), 1);
    //assert_eq!(hurl_log.entries.get(0).unwrap().response.status, 200);
//    assert!(hurl_log
//        .entries
//        .get(0)
//        .unwrap()
//        .asserts
//        .get(0)
//        .unwrap()
//        .clone()
//        .success());
}
