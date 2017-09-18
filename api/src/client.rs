use reqwest;
use reqwest::header::{Headers, ContentType, Accept};
use std::borrow::{Cow, Borrow};
use std::io::Read;
use url::Url;

use error::ClientError;

header! { (XRundeckAuthToken, "X-Rundeck-Auth-Token") => [String] }

#[derive(Clone)]
pub struct Client<'a> {
    inner: reqwest::Client,
    headers: Headers,
    url: Cow<'a, str>,
    trailing_slash: bool,
    pub api_version: i32
}

impl<'a> Client<'a> {

    /// Create a new ApiClient
    ///
    /// # Example
    /// ```
    /// use rundeck_api::client::Client;
    ///
    /// let _ = Client::new("http://localhost/api/12", "token").unwrap();
    /// ```
    pub fn new<U, T>(url: U, token: T) -> Result<Self, ClientError>
        where U: Into<Cow<'a, str>>,
              T: ToString
    {
        let inner = match reqwest::Client::new() {
            Ok(client) => client,
            Err(_) => return Err(ClientError::InternalClientCreation)
        };

        let mut headers = Headers::new();
        headers.set(Accept::json());
        headers.set(XRundeckAuthToken(token.to_string()));

        let url_saved = url.into();

        // Check if the url is parsable
        match Url::parse(url_saved.borrow()) {
            Err(_) => return Err(ClientError::MalformedUrl),

            // If path doesn't contain a version
            Ok(u) => if u.path().len() <= 1 {
                return Err(ClientError::MalformedUrl)
            }
        }

        let trailing_slash = url_saved.ends_with("/");

        // parse the api version
        let api_version: i32 = match url_saved.split("/").filter(|x| x.len() > 0 ).last() {
            Some(v) => match v.parse() {
                Ok(api) => api,
                Err(_) => return Err(ClientError::UncompatibleVersion),
            },
            None => return Err(ClientError::MalformedUrl)
        };

        Ok(Self {
            inner,
            headers,
            url: url_saved,
            trailing_slash,
            api_version
        })
    }

    /// Check the connectivity to the API (doesn't test the token)
    ///
    /// # Example
    /// ```
    /// extern crate mockito;
    /// extern crate rundeck_api;
    /// use rundeck_api::client::Client;
    ///
    ///
    /// fn main() {
    ///     let mock = mockito::mock("GET", "/12/system/info").with_status(204).create();
    ///     let _ = Client::new(format!("{}/12", mockito::SERVER_URL), "token").unwrap().check_connectivity();
    ///
    ///     mock.assert();
    ///     mockito::reset();
    /// }
    ///
    /// ```
    pub fn check_connectivity(&self) -> Result<(), ClientError> {
        let mut req = self.inner
            .request(reqwest::Method::Get, self.format_url("system/info", "")?)
            .expect("Unable to build the check http request");

        req.headers(self.headers.clone());

        match req.send() {
            Ok(r) => if r.status().is_success() || r.status().is_redirection() {
                Ok(())
            } else {
                Err(ClientError::Connectivity)
            },
            Err(_) => Err(ClientError::Connectivity)
        }
    }

    #[inline]
    fn format_url(&self, url: &str, query: &str) -> Result<Url, ClientError> {
        match Url::parse(&format!(
                "{}{}{}{}{}",
                self.url,
                // Add trailing slash if missing
                if self.trailing_slash { "" } else { "/" },
                url,
                // Add ? if query isn't empty
                if query.len() > 0 { "?" } else { "" },
                query
            )) {
            Ok(u) => Ok(u),
            Err(_) => return Err(ClientError::MalformedUrl)
        }
    }

    pub fn perform_get<S: ToString>(&self, url: &str, query: &mut Vec<S>) -> Result<String, ClientError> {
        let mut query_string = query.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        query_string.push("format=json".to_string());

        let query = query_string.join("&");

        let url = self.format_url(url, &query)?;

        let mut res = self.inner.get(url).unwrap()
            .headers(self.headers.clone())
            .send()
            .unwrap();

        let mut content = String::new();
        let _ = res.read_to_string(&mut content);

        Ok(content)
    }

    pub fn perform_post(&self, url: &str, body: &str) -> Result<String, ClientError> {
        let mut query_string: Vec<String> = Vec::new();

        query_string.push("format=json".to_string());

        let query = query_string.join("&");

        let url = self.format_url(url, &query)?;

        let mut headers = self.headers.clone();
        headers.set(ContentType::json());

        let mut res = self.inner.post(url).unwrap()
            .headers(headers)
            .body(body.to_string())
            .send()
            .unwrap();

        let mut content = String::new();
        let _ = res.read_to_string(&mut content);

        if res.status().is_success() {
            Ok(content)
        } else {
            match res.status() {
                reqwest::StatusCode::BadRequest => Err(ClientError::BadRequest(content)),
                _ => Ok(content)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate mockito;
    use super::*;

    #[test]
    fn new_success_trailing_slash() {
        match Client::new("http://localhost/12/", "azertyuop") {
            Err(_) => assert!(false),
            Ok(_) => {}
        };
    }

    #[test]
    fn new_success_no_trailing_slash() {
        match Client::new("http://localhost/12", "azertyuop") {
            Err(_) => assert!(false),
            Ok(_) => {}
        };
    }

    #[test]
    fn new_success_version() {
        match Client::new("http://localhost/12", "azertyuop") {
            Err(_) => assert!(false),
            Ok(_) => {}
        };
    }

    #[test]
    fn new_error_no_version() {
        match Client::new("http://localhost/", "azertyuop") {
            Err(e) => assert_eq!(e, ClientError::MalformedUrl),
            Ok(_) => assert!(false)
        };
    }

    #[test]
    fn new_error_uncompatible_version() {
        match Client::new("http://localhost/a12", "azertyuop") {
            Err(e) => assert_eq!(e, ClientError::UncompatibleVersion),
            Ok(_) => assert!(false)
        };
    }

    #[test]
    fn format_url_success() {
        match Client::new("http://localhost/12", "kk") {
            Ok(c) => assert_eq!(c.format_url("ok", "").unwrap().as_str(), "http://localhost/12/ok"),
            _ => assert!(false)
        }
    }

    #[test]
    fn format_url_success_with_query() {
        match Client::new("http://localhost/12", "kk") {
            Ok(c) => assert_eq!(c.format_url("ok", "").unwrap().as_str(), "http://localhost/12/ok"),
            _ => assert!(false)
        }
    }

    #[test]
    fn check_connectivity_success() {
        let mock = mockito::mock("GET", "/12/system/info")
            .with_status(200)
            .create();

        let result = match Client::new(format!("{}/12/", mockito::SERVER_URL), "token") {
            Ok(c) => {
                println!("hola");
                match c.check_connectivity() {
                    Ok(_) => true,
                    Err(_) => false
                }
            },
            _ => false
        };

        mock.assert();
        mockito::reset();
        assert!(result);
    }

    #[test]
    fn check_connectivity_error() {
        let mock = mockito::mock("GET", "/12/system/info")
            .with_status(404)
            .create();

        let result = match Client::new(format!("{}/12", mockito::SERVER_URL), "token") {
            Ok(c) => match c.check_connectivity() {
                Ok(_) => false,
                Err(_) => true
            },
            _ => false
        };

        mock.assert();
        mockito::reset();
        assert!(result);
    }
}
