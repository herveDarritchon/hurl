//use hurl::parser::error::ParseError;
use std;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::io::prelude::*;
use std::path::Path;

use atty::Stream;

use hurl::core::core::FormatError;
use hurl::parser;
use hurl::runner;
use hurl::runner::core::*;
use hurl::http;

fn execute(filename: &str,
           verbose: bool,
           insecure: bool,
           fail_fast: bool,
           output_color: bool,
           noproxy_hosts: Vec<String>,
           variables: &HashMap<String, String>,
           current_dir: &Path,
) -> HurlResult {
    let contents = if filename == "-" {
        let mut contents = String::new();
        io::stdin()
            .read_to_string(&mut contents)
            .expect("Something went wrong reading standard input");
        contents
    } else {
        if !Path::new(filename).exists() {
            eprintln!("Input file {} does not exit!", filename);
            std::process::exit(1);
        }
        fs::read_to_string(filename).expect("Something went wrong reading the file")
    };

    let mut lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(&contents)
        .collect();
    // edd an empty line at the end?
    lines.push("");

    let mut parser = parser::core::Parser::init(contents.as_str());
    match parser::parser::hurl_file(&mut parser) {
        Err(e) => {
            let lines = lines.iter().map(|s| s.to_string()).collect();
            let error = hurl::format::error::Error {
                exit_code: 1,
                source_info: e.source_info(),
                description: e.description(),
                fixme: e.fixme(),
                lines,
                filename: filename.to_string(),
                warning: false,
                color: output_color,
            };
            eprintln!("{}", error.format());
            println!("{} - parsing error", filename);
            std::process::exit(1);
        }
        Ok(hurl_file) => {
            if verbose {
                eprintln!("[DEBUG] no proxy for {:?}", noproxy_hosts);
                eprintln!("[DEBUG] Fail fast: {}", fail_fast);
                eprintln!("[DEBUG] variables: {:?}", variables);
            }

            let client = http::client::Client::init(http::client::ClientOptions {
                noproxy_hosts,
                insecure,
            });

            let context_dir = if filename == "-" {
                current_dir.to_str().unwrap()
            } else {
                let path = Path::new(filename);
                let parent = path.parent();
                parent.unwrap().to_str().unwrap()
            };
            let lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
            let hurl_result = runner::runner::run(client,
                                                  hurl_file,
                                                  fail_fast,
                                                  &variables,
                                                  verbose,
                                                  context_dir,
                                                  filename.to_string(),
                                                  output_color,
                                                  lines.clone(),
            );
            return hurl_result;
        }
    }
}

fn main() {
    let app = clap::App::new("hurl")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about("Run hurl FILE or standard input")
        .arg(
            clap::Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .multiple(true)
            //.index(1),
        )

        //
        // curl options
        //
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Turn on verbose output"),
        )
        .arg(
            clap::Arg::with_name("insecure")
                .short("k")
                .long("insecure")
                .help("Allow insecure SSl connections"),
        )

        //
        // hurl-specifcs
        //
        .arg(
            clap::Arg::with_name("color")
                .long("color")
                .conflicts_with("no-color")
                .help("Colorize Output"),
        )
        .arg(
            clap::Arg::with_name("no_color")
                .long("no-color")
                .conflicts_with("color")
                .help("Do not colorize Output"),
        )
        .arg(clap::Arg::with_name("noproxy")
            .long("noproxy")
            .value_name("HOST(S)")
            .help("List of hosts which do not use proxy")
            .takes_value(true)
        )
        .arg(clap::Arg::with_name("har_report")
            .long("har-report")
            .value_name("FILE")
            .help("Write output to har file")
            .takes_value(true)
        )
        .arg(clap::Arg::with_name("fail_at_end")
            .long("fail-at-end")
            .help("Fail at end")
            .takes_value(false)
        )
        .arg(clap::Arg::with_name("variable")
            .long("variable")
            .short("x")
            .value_name("NAME=VALUE")
            .multiple(true)
            .number_of_values(1)
            .help("Define a variable")
            .takes_value(true)
        );

    let matches = app.clone().get_matches();

    let output_color = if matches.is_present("color") {
        true
    } else if matches.is_present("no_color") {
        false
    } else {
        atty::is(Stream::Stdout)
    };


    let mut filenames: Vec<_> = match matches.values_of("INPUT") {
        None => vec![],
        Some(v) => v.collect()
    };

    // standard input
    if filenames.is_empty() {
        if atty::is(Stream::Stdin) {
            match app.clone().print_help() {
                Err(_) => panic!("error during print_help"),
                _ => {}
            }
            eprintln!("\n");
            std::process::exit(1);
        }
        filenames.push("-");
    }


    let mut variables = HashMap::new();
    if matches.is_present("variable") {
        let input: Vec<_> = matches.values_of("variable").unwrap().collect();
        for s in input {
            match s.find('=') {
                None => {
                    eprintln!("Missing variable value for {}!", s);
                    std::process::exit(1);
                }
                Some(index) => {
                    let (name, value) = s.split_at(index);
                    if variables.contains_key(name) {
                        std::process::exit(1);
                    }
                    variables.insert(name.to_string(), value[1..].to_string());
                }
            };
        }
    }

    let current_dir_buf = std::env::current_dir().unwrap();
    let current_dir = current_dir_buf.as_path();


    let verbose = matches.is_present("verbose");
    let insecure = matches.is_present("insecure");
    let fail_fast = !matches.is_present("fail_at_end");
    let noproxy_hosts: Vec<String> = match matches.value_of("noproxy") {
        Some(value) => {
            value.split(",").map(|e| e.trim().to_string()).collect()
        }
        _ => vec![]
    };

    let mut hurl_results = vec![];
    for filename in filenames {
        let hurl_result = execute(filename, verbose, insecure, fail_fast, output_color, noproxy_hosts.clone(), &variables, current_dir);
        hurl_results.push(hurl_result.clone());
    }

    if matches.is_present("har_report") {
        let har_file = matches.value_of("har_report").unwrap();
        let path = Path::new(har_file);


        let mut file = match std::fs::File::create(&path) {
            Err(why) => {
                eprintln!("Issue writing to {}: {:?}", har_file, why);
                std::process::exit(127)
            }
            Ok(file) => file,
        };

//let s = format!("{:#?}", hurl_logs);
//let hurl_logs = vec![1,2,3];
        let serialized = serde_json::to_string_pretty(&hurl_results).unwrap();
        match file.write_all(serialized.as_bytes()) {
            Err(why) => {
                eprintln!("Issue writing to {}: {:?}", har_file, why);
                std::process::exit(127)
            }
            Ok(_) => {}
        }
    }


    // Summary + exit code
    let mut count_errors_runner = 0;
    let mut count_errors_assert = 0;
    for hurl_result in hurl_results {
        let runner_errors: Vec<runner::core::Error> = hurl_result.clone().errors().iter().filter(|e| !e.assert).map(|e| e.clone()).collect();

        if hurl_result.clone().errors().is_empty() {
            println!("{} - success", hurl_result.filename);
        } else if runner_errors.is_empty() {
            println!("{} - assert error", hurl_result.filename);
            count_errors_assert += 1;
        } else {
            println!("{} - runner error", hurl_result.filename);
            count_errors_runner += 1;
        }
        let exit_code = if count_errors_runner > 0 {
            2
        } else if count_errors_assert > 0 {
            3
        } else {
            0
        };
        std::process::exit(exit_code as i32);
    }
}
