use super::Client;
use serde_json;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub group: Option<String>,
    pub project: String,
    pub href: String,
    pub permalink: String,
    pub description: String,
    pub schedule_enabled: Option<bool>,
    pub enabled: Option<bool>,
    pub scheduled: Option<bool>,
}
impl Job {
    pub fn name_with_group(&self) -> String {
        match self.group {
            Some(ref g) => format!("{}/{}", g, self.name),
            None => self.name.clone()
        }
    }
}

fn compile_filters(filters: Vec<&str>) -> Vec<String> {
    filters
        .iter()
        .map(|x|{
            let mut z = x.to_string();

            if z.starts_with("name") {
                z = format!("jobFilter={}", z.split("=").collect::<Vec<&str>>()[1]);
            } else if z.starts_with("group") {
                z = format!("groupPath={}", z.split("=").collect::<Vec<&str>>()[1]);
            }

            z
        })
        .collect::<Vec<String>>()
}

#[derive(Debug, Clone, Serialize)]
pub struct RunBody {
    pub filter: Option<String>,
    pub options: HashMap<String, String>,
    #[serde(rename="argString")]
    pub arg_string: Option<String>
}

#[derive(Clone)]
pub struct JobService<'a> {
    client: &'a Client<'a>
}

impl<'a> JobService<'a> {
    pub fn from_client(client: &'a Client) -> Result<Self, ()>
    {
        Ok(Self {
            client
        })
    }

    pub fn list(&self, project: &str, filters: Vec<&str>) -> Vec<Job> {
        let mut filters = compile_filters(filters);

        let ret = self.client.perform_get(&format!("project/{}/jobs",project), &mut filters);

        serde_json::from_str(&ret).unwrap()
    }

    pub fn run(&self, job: &str, body: &RunBody) {
        let mut body = body.clone();
        if self.client.api_version <= 18 {
            let mut arg_string: Vec<String> = Vec::new();
            for (name, value) in &body.options {
                arg_string.push(format!("-{} {}",name, value));
            }

            body.arg_string = Some(arg_string.join(" "));
        }
        let body = serde_json::to_string(&body).unwrap();
        self.client.perform_post(&format!("job/{}/run", job), &body);
    }
}
