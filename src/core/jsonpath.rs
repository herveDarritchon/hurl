// jsonpath
// unique entrypoint to external crate
// reference https://goessner.net/articles/JsonPath/index.html
// first examples not supported!


extern crate jsonpath;
extern crate serde_json;

use super::core::Value;

pub struct Expr {
    selector: jsonpath::Selector,
}

impl Expr {
    pub fn init(expression: &str) -> Option<Expr> {
        return match jsonpath::Selector::new(expression) {
            Ok(selector) => Some(Expr { selector }),
            Err(_) => None,
        };
    }

    pub fn eval(self, json: &str) -> Result<Value, serde_json::Error> {
        let root: serde_json::Value = serde_json::from_str(json)?;
        let values: Vec<&serde_json::Value> = self.selector.find(&root).collect();
        return Ok(Value::List(values.iter().filter_map(|e|to_value(e)).collect()));
    }
}


fn to_value(value: &serde_json::Value) -> Option<Value> {
    return match value {
        serde_json::Value::Null => None,
        serde_json::Value::Bool(bool) => Some(Value::Bool(*bool)),
        serde_json::Value::Number(n) => Some(
            if n.is_f64() {
                Value::from_f64(n.as_f64().unwrap())
            } else {
                Value::Integer(n.as_i64().unwrap())
            }
        ),
        serde_json::Value::String(s) => Some(Value::String(s.to_string())),
        serde_json::Value::Array(elements) => Some(
            Value::List(elements
                .iter()
                .filter_map(|e| to_value(e))
                .collect())
        ),
        serde_json::Value::Object(map) => {
            let mut elements = vec![];
            for (key, value) in map {
                match to_value(value) {
                    Some(value) => elements.push((key.to_string(), value)),
                    _ => {}
                }

            }
            Some(Value::Object(elements))
        },
    };
}

#[test]
// only query json object
// get scalar value or list length
fn test_to_value() {
    assert_eq!(to_value(&serde_json::from_str("null").unwrap()), None);
    assert_eq!(to_value(&serde_json::from_str("true").unwrap()).unwrap(), Value::Bool(true));
    assert_eq!(to_value(&serde_json::from_str("1").unwrap()).unwrap(), Value::Integer(1));
    assert_eq!(to_value(&serde_json::from_str("-1").unwrap()).unwrap(), Value::Integer(-1));
    assert_eq!(to_value(&serde_json::from_str("1.0").unwrap()).unwrap(), Value::from_f64(1.0));
    assert_eq!(to_value(&serde_json::from_str(r#""hello""#).unwrap()).unwrap(), Value::String(String::from("hello")));
    assert_eq!(to_value(&serde_json::from_str("[]").unwrap()).unwrap(), Value::List(vec![]));
    assert_eq!(to_value(&serde_json::from_str("[true,1,null]").unwrap()).unwrap(), Value::List(vec![
        Value::Bool(true), Value::Integer(1)
    ]));
    assert_eq!(to_value(&serde_json::from_str("{}").unwrap()).unwrap(), Value::Object(vec![]));
    assert_eq!(to_value(&serde_json::from_str(r#"{"name":"bob", "age": 32, "unused": null}"#).unwrap()).unwrap(), Value::Object(vec![
        (String::from("age"), Value::Integer(32)),
        (String::from("name"), Value::String(String::from("bob"))),

    ]));

}

#[test]
// only query json object
// get scalar value or list length
fn test_eval() {
    let json = r#"
{
  "result": {
    "success": false,
    "errors": [
      { "id": "error1" },
      { "id": "error2" }
    ]
  }
}
"#;

    // $.result.success
    // [ false  ]
    let expr = Expr::init("$.result.success").unwrap();
    assert_eq!(expr.eval(json.clone()).unwrap(), Value::List(vec![
        Value::Bool(false)
    ]));

    // $.result.errors
    // [ [ {"id": "error1" }, {"id": "error2" } ]]
    let expr = Expr::init("$.result.errors").unwrap();
    assert_eq!(expr.eval(json.clone()).unwrap(), Value::List(vec![
        Value::List(vec![
            Value::Object(vec![(String::from("id"), Value::String(String::from("error1")))]),
            Value::Object(vec![(String::from("id"), Value::String(String::from("error2")))]),
        ])
    ]));


    // $.result.errors[0].id
    // [ "error1" ]
    let expr = Expr::init("$.result.errors[0].id").unwrap();
    assert_eq!(expr.eval(json.clone()).unwrap(), Value::List(vec![
        Value::String(String::from("error1"))
    ]));
}


#[test]
fn test_reference_examples() {
    let _json: serde_json::Value = serde_json::from_str(r#"
{
    "store": {
        "book": [
            { "category": "reference",
                "author": "Nigel Rees",
                "title": "Sayings of the Century",
                "price": 8.95
            },
            { "category": "fiction",
                "author": "Evelyn Waugh",
                "title": "Sword of Honour",
                "price": 12.99
            },
            { "category": "fiction",
                "author": "Herman Melville",
                "title": "Moby Dick",
                "isbn": "0-553-21311-3",
                "price": 8.99
            },
            { "category": "fiction",
                "author": "J. R. R. Tolkien",
                "title": "The Lord of the Rings",
                "isbn": "0-395-19395-8",
                "price": 22.99
            }
        ],
        "bicycle": {
            "color": "red",
            "price": 19.95
        }
    }
}
"#).unwrap();

    // the authors of all books in the store (not supported)
    //let selector = jsonpath::Selector::new("$.store.book[*].author").unwrap();

    // all authors (not supported)
    //let selector = jsonpath::Selector::new("$..author");

    // all things in store, which are some books and a red bicycle.
    let _selector = jsonpath::Selector::new("$.store.*").unwrap();
    let _selector = jsonpath::Selector::new("$.statusCode").unwrap();

}
