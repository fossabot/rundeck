use client::Client;
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

    /// Create a new Job with an empty state
    /// Every string are empty, every boolean are false and all optional are None
    ///
    /// # Example
    /// ```
    /// use rundeck_api::job::Job;
    ///
    /// let j = Job::new_empty();
    ///
    /// assert_eq!(j.id, "");
    /// assert_eq!(j.name, "");
    /// assert_eq!(j.group, None);
    /// assert_eq!(j.project, "");
    /// assert_eq!(j.href, "");
    /// assert_eq!(j.permalink, "");
    /// assert_eq!(j.description, "");
    /// assert_eq!(j.schedule_enabled, None);
    /// assert_eq!(j.enabled, None);
    /// assert_eq!(j.scheduled, None);
    /// ```
    pub fn new_empty() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            group: None,
            project: String::new(),
            href: String::new(),
            permalink: String::new(),
            description: String::new(),
            schedule_enabled: None,
            enabled: None,
            scheduled: None,
        }
    }

    /// Return the job's name concatenated with his group.
    ///
    /// # Example
    /// ```
    /// use rundeck_api::job::Job;
    ///
    /// let mut j = Job::new_empty();
    /// j.name = "job_name".to_string();
    /// j.group = Some("group/name".to_string());
    ///
    /// assert_eq!(j.name_with_group(), "group/name/job_name");
    ///
    /// ```
    pub fn name_with_group(&self) -> String {
        match self.group {
            Some(ref g) => format!("{}/{}", g, self.name),
            None => self.name.clone()
        }
    }
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
    /// Create a new JobService with an instance of Client
    ///
    /// # Example
    /// ```
    /// use rundeck_api::client::Client;
    /// use rundeck_api::job::JobService;
    ///
    /// let client = Client::new("http://localhost/url/12", "token").unwrap();
    ///
    /// match JobService::from_client(&client) {
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

    pub fn list(&self, project: &str, filters: Vec<&str>) -> Vec<Job> {
        let mut filters = compile_filters(filters);

        let ret = self.client.perform_get(&format!("project/{}/jobs",project), &mut filters).unwrap();

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
        let _ = self.client.perform_post(&format!("job/{}/run", job), &body);
    }
}
