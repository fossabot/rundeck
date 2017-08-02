extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use reqwest::header::{Headers, ContentType, Accept};
use std::borrow::Cow;
use std::io::Read;

pub mod job;
mod project;

pub use self::job::{JobService, Job};
pub use self::project::{ProjectService, Project};

#[derive(Clone)]
pub struct Client<'a> {
    inner: reqwest::Client,
    headers: Headers,
    token: Cow<'a, str>,
    url: Cow<'a, str>,
    api_version: i32
}

impl<'a> Client<'a> {
    pub fn new<U, T>(url: U, token: T) -> Self 
        where U: Into<Cow<'a, str>>,
              T: Into<Cow<'a, str>>
    {
        let inner = reqwest::Client::new().expect("Cannot create an HTTP client");

        let mut headers = Headers::new();
        headers.set(Accept::json());


        let url_saved = url.into();
        let api_version: i32 = url_saved.split("/").filter(|x| x.len() > 0 ).last().unwrap().parse().unwrap();

        Self {
            inner,
            headers,
            url: url_saved,
            token: token.into(),
            api_version
        }
    }

    pub fn check_connectivity(&self) -> Result<reqwest::Response, reqwest::Error> {
        let mut req = self.inner.request(reqwest::Method::Options, &format!("{}", self.url)).expect("Unable to build the check http request");
        req.send()
    }

    fn perform_get<S: ToString>(&self, url: &str, query: &mut Vec<S>) -> String {
        let mut query_string = query.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        query_string.push(format!("authtoken={}", self.token));
        query_string.push("format=json".to_string());

        let query = query_string.join("&");

        let url = format!("{}{}?{}", self.url, url, query);

        let mut res = self.inner.get(&url).unwrap()
            .headers(self.headers.clone())
            .send()
            .unwrap();

        let mut content = String::new();
        let _ = res.read_to_string(&mut content);

        content
    }

    fn perform_post(&self, url: &str, body: &str) {
        let mut query_string: Vec<String> = Vec::new();

        query_string.push(format!("authtoken={}", self.token));
        query_string.push("format=json".to_string());

        let query = query_string.join("&");

        let url = format!("{}{}?{}", self.url, url, query);

        let mut headers = self.headers.clone();
        headers.set(ContentType::json());

        self.inner.post(&url).unwrap()
            .headers(headers)
            .body(body.to_string())
            .send()
            .unwrap();

    }
}
