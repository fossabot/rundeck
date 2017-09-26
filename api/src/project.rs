use client::Client;
use std::borrow::Cow;
use serde_json;
use error::ClientError;

// use prettytable::format;
// use prettytable::row::Row;
// use prettytable::cell::Cell;
// use std::io::Read;


#[derive(Clone, Debug, Deserialize)]
pub struct Project<'a> {
    pub url: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>
}

// fn compile_filters(filters: Vec<&str>) -> Vec<String> {
//     filters
//         .iter()
//         .map(|x|{
//             let mut z = x.to_string();

//             if z.starts_with("name") {
//                 z = format!("jobFilter={}", z.split("=").collect::<Vec<&str>>()[1]);
//             } else if z.starts_with("group") {
//                 z = format!("groupPath={}", z.split("=").collect::<Vec<&str>>()[1]);
//             }

//             z
//         })
//         .collect::<Vec<String>>()
// }

#[derive(Clone)]
pub struct ProjectService<'a> {
    client: &'a Client<'a>
}

impl<'a> ProjectService<'a> {
    pub fn from_client(client: &'a Client) -> Result<Self, ClientError>
    {
        Ok(Self {
            client
        })
    }

    pub fn list(&self) -> Vec<Project> {
        let mut filters: Vec<&str> = Vec::new();
        let ret = self.client.perform_get("projects", &mut filters).unwrap();

        serde_json::from_str(&ret).unwrap()
    }
}
