extern crate hurl;

use hurl::runner::http::*;


fn default_client_options() -> ClientOptions {
    return ClientOptions { noproxy_hosts: vec![],  insecure: true };
}

#[test]
fn test_hello() {
    let client = Client::init(default_client_options());

    let request = Request {
        method: Method::Get,
        url: Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/hello".to_string(),
            querystring: None
        }, //"http://localhost:8000/hello".to_string(),
        //querystring_params: vec![],
        headers: vec![
            Header { name: String::from("User-Agent"), value: String::from("hurl/0.1.1") },
            Header { name: String::from("Host"), value: String::from("TBD") }
        ],
        body: vec![],
    };

    let result = client.execute(&request);
    println!("{:?}", result);
    let response = result.unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.body.len(), 12);
    assert_eq!(
        String::from_utf8(response.body).unwrap(),
        "Hello World!".to_string()
    );
}

//#[test]
//fn test_text_utf8() {
//    let client = Client::init(ClientOptions {});
//
//    let request = Request {
//        method: Method::Get,
//        url: "http://localhost:5000/cafe".to_string(),
//        headers: vec![],
//        body: vec![],
//    };
//    let response = client.execute(&request).unwrap();
//    assert_eq!(response.status, 200);
//    assert_eq!(response.body.len(), 5);
//    assert_eq!(
//        String::from_utf8(response.body).unwrap(),
//        "café".to_string()
//    );
//}

#[cfg(test)]
fn hello_request() -> Request {
    Request {
        method: Method::Get,
        url: Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/hello".to_string(),
            querystring: None
        }, //"http://localhost:8000/hello".to_string(),
        //querystring_params: vec![],
        headers: vec![],
        body: vec![],
    }
}

#[test]
fn test_multiple_calls() {
    let client = Client::init(default_client_options());
    let response = client.execute(&hello_request()).unwrap();
    assert_eq!(response.status, 200);
    let response = client.execute(&hello_request()).unwrap();
    assert_eq!(response.status, 200);
}



#[test]
fn test_response_headers() {
    let client = Client::init(default_client_options());
    let response = client.execute(&hello_request()).unwrap();
    println!("{:?}", response);
    assert_eq!(response.status, 200);
    assert_eq!(
        response.get_header("Content-Length", false).unwrap(),
        "12".to_string()
    );
}

#[test]
fn test_send_cookie() {
    let client = Client::init(default_client_options());
    let request = Request {
        method: Method::Get,
        url: Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/cookies/set-request-cookie1-valueA".to_string(),
            querystring: None
        }, //"http://localhost:8000/send-cookie".to_string(),
        headers: vec![Header {
            name: "Cookie".to_string(),
            value: "cookie1=valueA;".to_string(),
        }],
        body: vec![],
    };
    let response = client.execute(&request).unwrap();
    assert_eq!(response.status, 200);

    let _client = Client::init(default_client_options());
    let _cookie_header = Header::from_cookies(vec![Cookie {
        name: "Cookie1".to_string(),
        value: "valueA;".to_string(),
        max_age: None
    }]);
    /*
    let request = Request {
        method: Method::Get,
        url: "http://localhost:5000/send-cookie1-value1".to_string(),
        headers: vec![cookie_header],
        body: vec![],
    };
    let response = client.execute(&request).unwrap();
    assert_eq!(response.status, 200);
    */
}

#[test]
fn test_redirect() {
    let client = Client::init(default_client_options());

    let request = Request {
        method: Method::Get,
        url: Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/redirect".to_string(),
            querystring: None
        }, // "http://localhost:8000/redirect".to_string(),
        headers: vec![],
        body: vec![],
    };
    let response = client.execute(&request).unwrap();
    assert_eq!(response.status, 302);
    assert_eq!(
        response.get_header("location", true).unwrap(),
        "http://redirectme".to_string()
    );
}

#[test]
fn test_querystring_param() {
    let client = Client::init(default_client_options());

    let request = Request {
        method: Method::Get,
        url: Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/querystring-params".to_string(),
            querystring: Some(String::from("param1=value1&param2&param3=a%3db"))
        },
        headers: vec![],
        body: vec![],
    };
    let response = client.execute(&request).unwrap();
    assert_eq!(response.status, 200);
}

#[test]
// curl -H 'Host:localhost:5000' -H 'content-type:application/x-www-form-urlencoded' -X POST 'http://localhost:5000/form-params' --data-binary 'param1=value1&param2='
fn test_form_param() {
    let client = Client::init(default_client_options());

    let request = Request {
        method: Method::Post,
        url: Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/form-params".to_string(),
            querystring: None
        }, // "http://localhost:8000/form-params".to_string(),
        //querystring_params: vec![],
        headers: vec![Header {
            name: "Content-Type".to_string(),
            value: "application/x-www-form-urlencoded".to_string(),
        }],
        body: "param1=value1&param2=".to_string().into_bytes(),
    };
    let response = client.execute(&request).unwrap();
    assert_eq!(response.status, 200);

    /*
        let client = Client::init(ClientOptions {}); // TO BE FIXED connection ended before message read => sync wait??
        let request = Request {
            method: Method::Post,
            url: "http://localhost:5000/form-params".to_string(),
            headers: vec![Header {
                name: "Content-Type".to_string(),
                value: "application/x-www-form-urlencoded".to_string(),
            }],
            body: encode_form_params(vec![
                Param {
                    name: "param1".to_string(),
                    value: "value1".to_string(),
                },
                Param {
                    name: "param2".to_string(),
                    value: "".to_string(),
                },
            ]),
        };
        let response = client.execute(&request).unwrap();
        assert_eq!(response.status, 200);
    */
}
