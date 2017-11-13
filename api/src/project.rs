use client::Client;
use std::borrow::Cow;
use serde_json;
use error::ClientError;

#[derive(Clone, Debug, Deserialize)]
pub struct Project<'a> {
    pub url: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
}

#[derive(Clone)]
pub struct ProjectService<'a> {
    client: &'a Client<'a>,
}

impl<'a> ProjectService<'a> {
    pub fn from_client(client: &'a Client) -> Result<Self, ClientError> {
        Ok(Self { client })
    }

    pub fn list(&self) -> Vec<Project> {
        let mut filters: Vec<String> = Vec::new();
        let ret = match self.client.perform_get("projects", &mut filters) {
            Ok(v) => v,
            Err(e) => panic!(e),
        };

        serde_json::from_str(&ret).unwrap()
    }
}
