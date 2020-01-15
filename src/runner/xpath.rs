// unique entry point to libxml
extern crate libxml;

use std::ffi::CStr;

use super::super::core::core::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XpathError {
    InvalidXML,
    InvalidHtml,
    Eval,
    Unsupported,
}

pub fn eval_xml(xml: String, expr: String) -> Result<Value, XpathError> {
    let parser = libxml::parser::Parser::default();
    return match parser.parse_string(xml) {
        Ok(doc) => if doc.get_root_element() == None {
            Err(XpathError::InvalidXML {})
        } else {
            eval(doc, expr)
        },
        Err(_) => Err(XpathError::InvalidXML {})
    };
}

pub fn eval_html(html: String, expr: String) -> Result<Value, XpathError> {
    let parser = libxml::parser::Parser::default_html();
    return match parser.parse_string(html) {
        Ok(doc) => {
            // You can have a doc structure even if the input xml is not valid
            // check that the root element exists
            if doc.get_root_element() == None {
                Err(XpathError::InvalidHtml {})
            } else {
                eval(doc, expr)
            }
        }
        Err(_) => Err(XpathError::InvalidHtml {})
    };
}

pub fn eval(doc: libxml::tree::Document, expr: String) -> Result<Value, XpathError> {
    //let parser = libxml::parser::Parser::default();
//    let doc = match parser.parse_string(xml) {
//        Ok(doc) => doc,
//        Err(_) => return Err(XpathError::InvalidXML {})
//    };


    let context = match libxml::xpath::Context::new(&doc) {
        Ok(context) => context,
        Err(_) => panic!("error setting context in xpath module")
    };
    unsafe {
        libxml::bindings::initGenericErrorDefaultFunc(&mut None);
    }
    let result = match context.evaluate(expr.as_str()) {
        Ok(object) => {
            object
        }
        Err(_) => return Err(XpathError::Eval {})
    };
    // eprintln!("result> {}", result);
    return match unsafe { *result.ptr }.type_ {
        libxml::bindings::xmlXPathObjectType_XPATH_NUMBER => Ok(Value::from_f64(unsafe { *result.ptr }.floatval)),
        libxml::bindings::xmlXPathObjectType_XPATH_BOOLEAN =>
            Ok(Value::Bool(unsafe { *result.ptr }.boolval != 0)),
        libxml::bindings::xmlXPathObjectType_XPATH_STRING => {
            // TO BE CLEANED
            let c_s = unsafe { *result.ptr }.stringval;
            let c_s2 = c_s as *const i8;
            let x = unsafe { CStr::from_ptr(c_s2) };
            let s = x.to_string_lossy().to_string();

            Ok(Value::String(s))
        }
        libxml::bindings::xmlXPathObjectType_XPATH_NODESET => Ok(Value::Nodeset(result.get_number_of_nodes())),
        _ => {
            Err(XpathError::Unsupported {})
        }
    };
}


#[test]
fn test_xml() {
    let xml = String::from(r#"<?xml version="1.0" encoding="utf-8"?>
<food>
  <banana type="fruit" price="1.1"/>
  <apple type="fruit"/>
  <beef type="meat"/>
</food>
"#);
    let xpath = String::from("count(//food/*)");
    assert_eq!(eval_xml(xml.clone(), xpath).unwrap(), Value::from_f64(3.0));

    let xpath = String::from("//food/*");
    assert_eq!(eval_xml(xml.clone(), xpath).unwrap(), Value::Nodeset(3));

    let xpath = String::from("count(//*[@type='fruit'])");
    assert_eq!(eval_xml(xml.clone(), xpath).unwrap(), Value::from_f64(2.0));

    let xpath = String::from("number(//food/banana/@price)");
    assert_eq!(eval_xml(xml.clone(), xpath).unwrap(), Value::from_f64(1.1));
}


#[test]
fn test_error_eval() {
    assert_eq!(eval_xml(String::from("<a/>"), String::from("^^^")).err().unwrap(), XpathError::Eval {});
    assert_eq!(eval_xml(String::from("<a/>"), String::from("//")).err().unwrap(), XpathError::Eval {});
    // assert_eq!(1,2);
}


// TBC!!!
// Invalid XML not detected at parsing??? => goes into an eval error
#[test]
fn test_invalid_xml() {
    assert_eq!(eval_xml(String::from("??"), String::from("//person")).err().unwrap(), XpathError::InvalidXML);
}

#[test]
fn test_cafe() {
    //println!("{:?}", eval( String::from("<data>café</data>"), String::from("normalize-space(//data)")));
    assert_eq!(eval_xml(
        String::from("<data>café</data>"), String::from("normalize-space(//data)")).unwrap(),
               Value::String(String::from("café"))
    );
}


#[test]
fn test_html() {
    let html = String::from(r#"<html>
  <head>
    <meta charset="UTF-8"\>
  </head>
  <body>
    <br>
  </body>
</html>"#);
    let xpath = String::from("normalize-space(/html/head/meta/@charset)");
    assert_eq!(eval_html(html.clone(), xpath).unwrap(), Value::String(String::from("UTF-8")));
}


#[test]
fn test_bug() {
    let html = String::from(r#"<html></html>"#);
    //let xpath = String::from("boolean(count(//a[contains(@href,'xxx')]))");
    let xpath = String::from("boolean(count(//a[contains(@href,'xxx')]))");
    assert_eq!(eval_html(html.clone(), xpath).unwrap(), Value::Bool(false));
}

