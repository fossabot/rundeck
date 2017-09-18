use client::Client;
use serde_json;
use std::collections::HashMap;
use error::ClientError;

#[derive(Clone, Debug, Deserialize)]
pub struct Token {
    pub user: String,
    pub id: String,
    creator: String,
    expiration: String,
    roles: Vec<String>,
    expired: bool
}

impl Token {
}

/// Compile filters
/// TODO: Rewrite this shit
///
/// # Example
/// ```
/// use rundeck_api::job::compile_filters;
/// assert_eq!(compile_filters(vec!()), Vec::new() as Vec<String>);
/// ```
pub fn compile_filters(filters: Vec<&str>) -> Vec<String> {
    filters
        .iter()
        .map(|x|{
            let mut z = x.to_string();
            z
        })
        .collect::<Vec<String>>()
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenBody {
    pub user: String,
    pub roles: Vec<String>,
    pub duration: String
}

#[derive(Clone)]
pub struct TokenService<'a> {
    client: &'a Client<'a>
}

#[derive(Clone, Debug, Deserialize)]
pub struct TokenCreationError {
    error: bool,
    apiversion: i16,
    errorCode: String,
    pub message: String
}

impl<'a> TokenService<'a> {
    /// Create a new TokenService with an instance of Client
    ///
    /// # Example
    /// ```
    /// use rundeck_api::client::Client;
    /// use rundeck_api::job::TokenService;
    ///
    /// let client = Client::new("http://localhost/url/12", "token").unwrap();
    ///
    /// match TokenService::from_client(&client) {
    ///     Ok(_) => assert!(true),
    ///     Err(_) => assert!(false)
    /// }
    /// ```
    pub fn from_client(client: &'a Client) -> Result<Self, ()>
    {
        Ok(Self {
            client
        })
    }

    pub fn list(&self, filters: Vec<&str>) -> Vec<Token> {
        let mut filters = compile_filters(filters);

        let ret = self.client.perform_get("tokens", &mut filters).unwrap();

        serde_json::from_str(&ret).unwrap()
    }

    pub fn new(&self, body: &TokenBody) -> Result<Token, TokenCreationError> {
        match self.client.perform_post("tokens", &serde_json::to_string(&body).unwrap()) {
            Ok(ret) => Ok(serde_json::from_str(&ret).unwrap()),
            Err(ret) => Err(match ret {
                ClientError::BadRequest(s) => match serde_json::from_str(&s) {
                    Ok(ret) => ret,
                    Err(_) => TokenCreationError {
                        error: true,
                        apiversion: 0,
                        errorCode: "".to_string(),
                        message: "".to_string()
                    }
                },
                _ => TokenCreationError {
                    error: true,
                    apiversion: 0,
                    errorCode: "".to_string(),
                    message: "".to_string()
                }
            })
        }
    }
}
