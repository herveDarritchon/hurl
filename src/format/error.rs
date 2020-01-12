use crate::core::core::SourceInfo;

use super::color::TerminalColor;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub exit_code: usize,
    pub source_info: SourceInfo,
    pub description: String,
    pub fixme: String,
    pub lines: Vec<String>,
    pub filename: String,
    pub warning: bool,
    pub color: bool,
}


impl Error {
    pub fn format(self) -> String {
        let mut s = "".to_string();

        let error_type = if self.warning {
            String::from("warning")
        } else {
            String::from("error")
        };
        let error_type = if !self.color {
            error_type
        } else if self.warning {
            TerminalColor::Yellow.format(error_type)
        } else {
            TerminalColor::Red.format(error_type)
        };
        s.push_str(format!("{}: {}\n", error_type, self.description).as_str());

        if self.filename != "-" {
            s.push_str(
                format!(
                    "  --> {}:{}:{}\n",
                    self.filename, self.source_info.start.line, self.source_info.start.column,
                )
                    .as_str(),
            );
        }
        s.push_str("   |\n");

        let line = self.lines.get(self.source_info.start.line - 1).unwrap();
        let width = (self.source_info.end.column - self.source_info.start.column) as usize;

        let mut tab_shift = 0;
        for (i, c) in line.chars().enumerate() {
            if i >= self.source_info.start.column - 1 { break; };
            if c == '\t' {
                tab_shift += 1;
            }
        }

        let line = str::replace(line, "\t", "    ");    // replace all your tabs with 4 characters
        s.push_str(
            format!(
                "{line_number:>width$} |{line}\n",
                line_number = self.source_info.start.line,
                width = 2,
                line = if line.is_empty() { line.clone() } else { format!(" {}", line) }
            )
                .as_str(),
        );


        s.push_str(
            format!(
                "   | {}{} {fixme}\n",
                " ".repeat(self.source_info.start.column - 1 + tab_shift * 3),
                "^".repeat(if width > 1 { width } else { 1 }),
                fixme = self.fixme.as_str(),
            )
                .as_str(),
        );
        s.push_str("   |\n");

        return s.to_string();
    }
}


#[test]
fn test_basic() {
    let filename = String::from("integration/hurl_error_lint/spaces.hurl");
    let lines = vec![
        String::from("GET\thttp://localhost:8000/hello")
    ];
    let error = Error {
        exit_code: 0,
        source_info: SourceInfo::init(1, 4, 1, 5),
        description: String::from("One space"),
        fixme: String::from("Use only one space"),
        lines,
        filename,
        warning: true,
        color: false,
    };
    assert_eq!(error.format(),
               String::from(r#"warning: One space
  --> integration/hurl_error_lint/spaces.hurl:1:4
   |
 1 | GET    http://localhost:8000/hello
   |    ^ Use only one space
   |
"#)
    );
}

#[test]
fn test_with_tabs() {
    let filename = String::from("integration/hurl_error_lint/spaces.hurl");
    let lines = vec![
        String::from("GET\thttp://localhost:8000/hello ")
    ];
    let error = Error {
        exit_code: 0,
        source_info: SourceInfo::init(1, 32, 1, 32),
        description: String::from("Unnecessary space"),
        fixme: String::from("Remove space"),
        lines,
        filename,
        warning: true,
        color: false,
    };
    assert_eq!(error.format(),
               concat!(
"warning: Unnecessary space\n",
"  --> integration/hurl_error_lint/spaces.hurl:1:32\n",
"   |\n",
" 1 | GET    http://localhost:8000/hello \n",
"   |                                   ^ Remove space\n",
"   |\n")
    );
}


#[test]
fn test_end_of_file() {

    // todo: improve error location

    let filename = String::from("hurl_error_parser/json_unexpected_eof.hurl");
    let lines = vec![
        String::from("POST http://localhost:8000/data\n"),
        String::from("{ \"name\":\n"),
        String::from("")
    ];
    let error = Error {
        exit_code: 0,
        source_info: SourceInfo::init(3, 1, 3, 1),
        description: String::from("Parsing json"),
        fixme: String::from("json error"),
        lines,
        filename,
        warning: true,
        color: false,
    };
    assert_eq!(error.format(),
               String::from(r#"warning: Parsing json
  --> hurl_error_parser/json_unexpected_eof.hurl:3:1
   |
 3 |
   | ^ json error
   |
"#)
    );
}