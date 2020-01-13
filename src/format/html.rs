use super::super::core::ast::*;

pub trait Htmlable {
    fn to_html(&self) -> String;
}

pub fn format_standalone(hurl_file: HurlFile) -> String {
    let css = include_str!("hurl.css");

    let mut buffer = String::from("");
    buffer.push_str("<!DOCTYPE html>\n");
    buffer.push_str("<html>");
    buffer.push_str("<head>");
    buffer.push_str("<title>Hurl File</title>");
    buffer.push_str("<style>\n");
    buffer.push_str(css);
    buffer.push_str("</style>");
    buffer.push_str("</head>");
    buffer.push_str("<body>\n");
    buffer.push_str(hurl_file.to_html().as_str());
    buffer.push_str("\n</body>");
    buffer.push_str("</html>");
    return buffer;
}

pub fn format(hurl_file: HurlFile, standalone: bool) -> String {
    return if standalone { format_standalone(hurl_file) } else { hurl_file.to_html() };
}

// region hurl-file
impl Htmlable for HurlFile {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<div class=\"hurl-file\">");
        for entry in self.clone().entries {
            buffer.push_str(entry.to_html().as_str());
        }
        for line_terminator in self.line_terminators.clone() {
            buffer.push_str(line_terminator.to_html().as_str());
        }
        buffer.push_str("</div>");
        return buffer;
    }
}
// endregion

// region hurl-entry
impl Htmlable for Entry {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<div class=\"hurl-entry\">");
        buffer.push_str(self.request.to_html().as_str());
        match self.clone().response {
            Some(response) => {
                buffer.push_str(response.to_html().as_str());
            }
            _ => {}
        }
        buffer.push_str("</div>");
        return buffer;
    }
}
// endregion

// region request
impl Htmlable for Request {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<div class=\"request\">");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.method.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str(self.url.to_html().as_str());
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str("</div>");
        for header in self.headers.clone() {
            buffer.push_str(header.to_html().as_str());
        }
        for section in self.sections.clone() {
            buffer.push_str(section.to_html().as_str());
        }
        return buffer;
    }
}
// endregion

// region response
impl Htmlable for Response {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<div class=\"response\">");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.version.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str(self.status.to_html().as_str());
        buffer.push_str("</span>");
        for section in self.sections.clone() {
            buffer.push_str(section.to_html().as_str());
        }
        buffer.push_str("</div>");
        return buffer;
    }
}
// endregion

// region method
impl Htmlable for Method {
    fn to_html(&self) -> String {
        return format!("<span class=\"method\">{}</span>", self.as_str());
    }
}
// endregion

// region version
impl Htmlable for Version {
    fn to_html(&self) -> String {
        return format!(
            "<span class=\"version\">HTTP/{}</span>",
            self.value.as_str()
        );
    }
}
// endregion

// region status
impl Htmlable for Status {
    fn to_html(&self) -> String {
        return format!("<span class=\"status\">{}</span>", self.value.to_string());
    }
}
// endregion

// region header
impl Htmlable for Header {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.name.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str("</span>");
        return buffer;
    }
}
// endregion

// region section
impl Htmlable for Section {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str(self.space0.to_html().as_str());

        buffer
            .push_str(format!("<span class=\"section-header\">[{}]</span>", self.name()).as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.value.to_html().as_str());
        return buffer;
    }
}

// endregion

// region section-value
impl Htmlable for SectionValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self {
            SectionValue::Asserts(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
            SectionValue::QueryParams(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
            SectionValue::FormParams(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
            SectionValue::Cookies(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
            SectionValue::Captures(items) => {
                for item in items {
                    buffer.push_str(item.to_html().as_str())
                }
            }
        }
        return buffer;
    }
}
// endregion

//// region section-capture
//impl Htmlable for Section<Capture> {
//    fn to_html(&self) -> String {
//        let mut buffer = String::from("");
//        add_line_terminators(&mut buffer, self.line_terminators.clone());
//        buffer.push_str(self.space0.to_html().as_str());
//        add_section_header(&mut buffer, self.name.clone());
//        for item in self.items.clone() {
//            buffer.push_str(item.to_html().as_str());
//        }
//
//        return buffer;
//    }
//}
//// endregion

//// region section-asserts
//impl Htmlable for Section<Assert> {
//    fn to_html(&self) -> String {
//        let mut buffer = String::from("");
//        add_line_terminators(&mut buffer, self.line_terminators.clone());
//        buffer.push_str(self.space0.to_html().as_str());
//        add_section_header(&mut buffer, self.name.clone());
//        for item in self.items.clone() {
//            buffer.push_str(item.to_html().as_str());
//        }
//        return buffer;
//    }
//}
//// endregion

// region param
impl Htmlable for Param {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.name.value.as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer.push_str("</span>");
        return buffer;
    }
}
// endregion

// region cookie
impl Htmlable for Cookie {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.name.value.as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.value.to_html().as_str());
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer.push_str("</span>");
        return buffer;
    }
}
// endregion

// region cookie
impl Htmlable for CookieValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str(self.value.as_str());
        return buffer;
    }
}
// endregion

// region capture
impl Htmlable for Capture {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.name.value.as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str("<span>:</span>");
        buffer.push_str(self.space2.to_html().as_str());
        buffer.push_str(self.query.to_html().as_str());
        buffer.push_str(self.line_terminator0.to_html().as_str());
        buffer.push_str("</span>");
        return buffer;
    }
}
// endregion



// region query

impl Htmlable for Query {
    fn to_html(&self) -> String {
        return self.value.to_html();
    }
}
impl Htmlable for QueryValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self {
            QueryValue::Status {} => {
                buffer.push_str("<span class=\"query-type\">status</span>");
            }
            QueryValue::Header { space0, name } => {
                buffer.push_str("<span class=\"query-type\">header</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(name.to_html().as_str());
            }
            QueryValue::Cookie { space0, name } => {
                buffer.push_str("<span class=\"query-type\">cookie</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(name.to_html().as_str());
            }
            QueryValue::Body {} => {
                buffer.push_str("<span class=\"query-type\">status</span>");
            }
            QueryValue::Xpath { space0, expr } => {
                buffer.push_str("<span class=\"query-type\">xpath</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(expr.to_html().as_str());
            }
            QueryValue::Jsonpath { space0, expr } => {
                buffer.push_str("<span class=\"query-type\">jsonpath</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(expr.to_html().as_str());
            }
        }

        return buffer;
    }
}
// endregion

// region assert
impl Htmlable for Assert {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        add_line_terminators(&mut buffer, self.line_terminators.clone());
        buffer.push_str("<span class=\"line\">");
        buffer.push_str(self.space0.to_html().as_str());
        buffer.push_str(self.query.to_html().as_str());
        buffer.push_str(self.space1.to_html().as_str());
        buffer.push_str(self.predicate.to_html().as_str());
        buffer.push_str("</span>");
        buffer.push_str(self.line_terminator0.to_html().as_str());
        return buffer;
    }
}
// endregion

// region predicate
impl Htmlable for Predicate {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        if self.not {
            buffer.push_str("not");
            buffer.push_str(self.space0.to_html().as_str());
        }
        buffer.push_str(self.predicate_func.to_html().as_str());
        return buffer;
    }
}
//endregion

// region predicate-func
impl Htmlable for PredicateFunc {
    fn to_html(&self) -> String {
        return self.value.to_html();
    }
}
//endregion

// region predicate-func
impl Htmlable for PredicateFuncValue {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self {
            PredicateFuncValue::CountEqual { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value).as_str());
            }
            PredicateFuncValue::EqualString {
                space0: _,
                value: _,
            } => {}
            PredicateFuncValue::EqualInt { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value).as_str());
            }
            PredicateFuncValue::EqualFloat { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(
                    format!(
                        "<span class=\"number\">{}</span>",
                        value.to_string()
                    )
                    .as_str(),
                );
            }
            PredicateFuncValue::EqualBool { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"boolean\">{}</span>", value).as_str());
            }
            PredicateFuncValue::StartWith {
                space0: _,
                value: _,
            } => {}
            PredicateFuncValue::Contain {
                space0: _,
                value: _,
            } => {}
            PredicateFuncValue::Match {
                space0: _,
                value: _,
            } => {}
            PredicateFuncValue::FirstEqualInt { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value).as_str());
            }
            PredicateFuncValue::FirstEqualBool { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value).as_str());
            }
            PredicateFuncValue::FirstEqualString { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">equals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"string\">{}</span>", value.to_html()).as_str());
            }
            PredicateFuncValue::FirstCountEqual { space0, value } => {
                buffer.push_str("<span class=\"predicate-type\">countEquals</span>");
                buffer.push_str(space0.to_html().as_str());
                buffer.push_str(format!("<span class=\"number\">{}</span>", value).as_str());
            }
            PredicateFuncValue::Exist { } => {
                buffer.push_str("<span class=\"predicate-type\">exists</span>");
            }
        }
        return buffer;
    }
}
// endregion

// region whitespace
impl Htmlable for Whitespace {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self {
            Whitespace { value, .. } => {
                if !value.is_empty() {
                    buffer.push_str(self.value.as_str());
                }
            }
        }
        return buffer;
    }
}
// endregion

// region line-terminator
impl Htmlable for LineTerminator {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        buffer.push_str(self.space0.to_html().as_str());
        match self.clone().comment {
            Some(v) => {
                buffer.push_str("<span class=\"comment\">");
                buffer.push_str(format!("#{}", v.value.as_str()).as_str());
                buffer.push_str("</span>");
            }
            _ => {}
        }
        return buffer;
    }
}
// endregion

// region hurl-string
impl Htmlable for HurlString {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        match self {
            HurlString {
                value,
                encoded: None,
                ..
            } => {
                if value.len() > 0 {
                    buffer.push_str(format!("<span class=\"string\">{}</span>", value).as_str());
                }
            }
            HurlString {
                encoded: Some(value),
                ..
            } => {
                buffer.push_str(format!("<span class=\"string\">\"{}\"</span>", value).as_str());
            }
        }
        return buffer;
    }
}
// endregion

// region hurl-template
impl Htmlable for HurlTemplate {
    fn to_html(&self) -> String {
        let mut buffer = String::from("");
        for element in self.elements.clone() {
            buffer.push_str(element.to_html().as_str());
        }
        return buffer;
    }
}

impl Htmlable for HurlTemplateElement {
    fn to_html(&self) -> String {
        return match self {
            HurlTemplateElement::Literal { value } => {
                format!("<span class=\"string\">{}</span>", value.value)
            }
            HurlTemplateElement::Expression { value } => value.to_html(),
            /*            space0: _, variable: _, space1: _ } => {
                let mut buffer = String::from("");
                buffer.push_str("{{");
                buffer.push_str("}}");
                return buffer;
            }*/
        };
    }
}
// endregion

// region expr
impl Htmlable for Expr {
    fn to_html(&self) -> String {
        return format!("<span class=\"variable\">{}</span>", self.variable.name)

    }
}
// endregion

// region helpers
fn to_line(v: String) -> String {
    format!("<span class=\"line\">{}</span>", v)
}

fn add_line_terminators(buffer: &mut String, line_terminators: Vec<LineTerminator>) {
    for line_terminator in line_terminators.clone() {
        buffer.push_str(to_line(line_terminator.to_html()).as_str());
    }
}

//fn add_section_header(buffer: &mut String, name: String) {
//    buffer.push_str("<span class=\"line\">");
//    buffer.push_str(format!("<span class=\"section-header\">[{}]</span>", name).as_str());
//    buffer.push_str("</span>");
//}
// endregion
