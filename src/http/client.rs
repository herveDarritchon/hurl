use super::core::*;
use super::request::*;
use super::response::*;


// TODO create http-specific error
use crate::runner::core::RunnerError;

pub struct Client {
    inner_client: reqwest::Client,
    _options: ClientOptions,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientOptions {
    pub noproxy_hosts: Vec<String>,
    pub insecure: bool,
}


impl Client {
    pub fn init(options: ClientOptions) -> Client {

//let mut headers = reqwest::header::HeaderMap::new();
//let user_agent = format!("hurl/{}",clap::crate_version!());
//headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_str(user_agent.as_str()).unwrap());
//headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static(""));
//eprintln!("{}", clap::crate_version!());

        let client_builder = reqwest::Client::builder()
            .redirect(reqwest::RedirectPolicy::none())
//.default_headers(headers)
            .use_sys_proxy()
            .danger_accept_invalid_hostnames(options.insecure)
            .danger_accept_invalid_certs(options.insecure)
//.http1_title_case_headers()
            .cookie_store(false);
        return Client {
            inner_client: client_builder.build().unwrap(),
            _options: options,
        };
    }

    //pub fn execute(&self, request: &Request) -> Result<Response, Error> {
    pub fn execute(&self, request: &Request) -> Result<Response, RunnerError> {
        let mut headers = reqwest::header::HeaderMap::new();
        for header in request.clone().headers {
            headers.insert(
                reqwest::header::HeaderName::from_lowercase(
                    header.name.to_lowercase().as_str().as_bytes(),
                )
                    .unwrap(),
                reqwest::header::HeaderValue::from_str(header.value.as_str()).unwrap(),
            );
        }


        // clear cookies from client
        // use only yours!


//        let url = if request.querystring_params.is_empty() {
//            request.clone().url
//        } else {
//            let params: Vec<String> = request.querystring_params
//                .iter()
//                .map(|p| format!("{}={}", p.name, p.value))
//                .collect();
//            format!("{}?{}", request.clone().url, params.join("&"))
//        };


        // no proxy variable depends on the request
        // rebuilding the client => you will need to keep the cookie yourself
//        let client_builder = reqwest::Client::builder()
//            .redirect(reqwest::RedirectPolicy::none())
//            .use_sys_proxy()
//            .cookie_store(true);
//
//        let client_builder = if self.options.noproxy_hosts.contains(&request.url.host.clone()) {
//            client_builder.no_proxy()
//        } else {
//            client_builder
//        };
//        let client = client_builder.build().unwrap();
        let client = &self.clone().inner_client;


        let req = client
            .request(
                request.clone().method.to_reqwest(),
                reqwest::Url::parse(request.clone().url().as_str()).unwrap(),
            )
            .headers(headers)
            .body(request.clone().body)
            .build()
            .unwrap();


        match client
            .execute(req) {
            Ok(mut resp) => {
                let mut headers = vec![];
                //eprintln!(">>> response headers {:?}", resp.headers().clone());
                for (name, value) in resp.headers() {
                    headers.push(Header {
                        name: name.as_str().to_string(),
                        value: value.to_str().unwrap().to_string(),
                    })
                }

                let version = match resp.version() {
                    reqwest::Version::HTTP_10 => Version::Http10,
                    reqwest::Version::HTTP_11 => Version::Http11,
                    reqwest::Version::HTTP_2 => Version::Http2,
                    v => panic!("Version {:?} not supported!", v),
                };
                let mut buf: Vec<u8> = vec![];
                resp.copy_to(&mut buf).unwrap();
                resp.content_length(); // dirty hack to prevent error "connection closed before message completed"?


                // extract cookies
                //eprintln!("[DEBUG] Cookies");
                //for cookie in resp.cookies() {
                //eprintln!("[DEBUG]  {}", cookie.name());
                //}
                return Ok(Response {
                    version,
                    status: resp.status().as_u16(),
                    headers,
                    body: buf,
                });
            }
            Err(e) => {
                return Err(RunnerError::HttpConnection {
                    message: format!("{:?}", e.to_string()),
                    url: request.clone().url(),
                });
            }
        }
    }
}
