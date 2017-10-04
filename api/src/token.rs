use std::borrow::Cow;
use std::ops::Deref;
use client::Client;
use serde_json;
use error::{ClientError, ApiError};

#[derive(Clone, Debug, Deserialize)]
pub struct Token<'a> {
    pub user: Cow<'a, str>,
    pub id: Cow<'a, str>,
    pub creator: Cow<'a, str>,
    expiration: Cow<'a, str>,
    roles: Vec<Cow<'a, str>>,
    pub expired: bool,
    pub token: Option<Cow<'a, str>>
}

/// Compile filters
/// TODO: Rewrite this shit
///
/// # Example
/// ```
/// use rundeck_api::job::compile_filters;
/// assert_eq!(compile_filters(vec!()), Vec::new() as Vec<String>);
/// ```
pub fn compile_filters<'a, I>(filters: &I) -> Vec<String>
    where I: Deref<Target=[&'a str]> + IntoIterator<Item=&'a str>
{
    filters
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenBody<'a> {
    pub user: Cow<'a, str>,
    pub roles: Vec<Cow<'a, str>>,
    pub duration: Cow<'a, str>
}

#[derive(Clone)]
pub struct TokenService<'a> {
    client: &'a Client<'a>
}

impl<'a> TokenService<'a> {
    /// Create a new TokenService with an instance of Client
    ///
    /// # Example
    /// ```
    /// use rundeck_api::client::Client;
    /// use rundeck_api::token::TokenService;
    ///
    /// let client = Client::new("http://localhost/url/12", "token").unwrap();
    ///
    /// match TokenService::from_client(&client) {
    ///     Ok(_) => assert!(true),
    ///     Err(_) => assert!(false)
    /// }
    /// ```
    pub fn from_client(client: &'a Client) -> Result<Self, ClientError>
    {
        Ok(Self {
            client
        })
    }

    pub fn list<I>(&self, filters: I) -> Vec<Token>
        where I: Deref<Target=[&'a str]> + IntoIterator<Item=&'a str>
    {
        let mut filters = compile_filters(&filters);

        let ret = self.client.perform_get("tokens", &mut filters).unwrap();

        serde_json::from_str(&ret).unwrap()
    }

    pub fn get(&self, t: &Token) -> Result<Token, ApiError> {
        let mut filters: Vec<String> = Vec::new();

        let ret = self.client.perform_get(&format!("token/{}", t.id), &mut filters).unwrap();

        let t: Token = serde_json::from_str(&ret).unwrap();

        Ok(t)
    }

    pub fn create(&self, body: &TokenBody) -> Result<Token, ApiError> {
        match self.client.perform_post("tokens", &serde_json::to_string(&body).unwrap()) {
            Ok(ret) => Ok(serde_json::from_str(&ret).unwrap()),
            Err(ret) => Err(match ret {
                ClientError::BadRequest(s) => match serde_json::from_str(&s) {
                    Ok(ret) => ret,
                    Err(_) => ApiError::new(true, 0, "", "")
                },
                _ => ApiError::new(true, 0, "", "")
            })
        }
    }
}
