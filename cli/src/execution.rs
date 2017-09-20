use std::borrow::Cow;
use api::Job;

#[derive(Clone, Debug, Deserialize)]
pub struct DateField<'a> {
    pub date: Cow<'a, str>
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExecutionPagination {
    pub executions: Vec<Execution>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Execution<'a> {
    pub id: i32,
    pub href: Cow<'a, str>,
    pub permalink: Cow<'a, str>,
    pub status: Cow<'a, str>,
    pub project: Cow<'a, str>,
    pub user: Cow<'a, str>,
    #[serde(rename = "date-started")]
    pub date: DateField,
    pub job: Job,
    pub description: Cow<'a, str>
}

// pub fn kill(client: &Client, url: &str, token: &str, execution_id: &str) {

// }
