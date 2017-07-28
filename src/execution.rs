use super::Job;

use reqwest::Client;
use serde_json;

use prettytable::format;
use prettytable::row::Row;
use prettytable::cell::Cell;
use std::io::Read;
use super::construct_headers;

#[derive(Clone, Debug, Deserialize)]
pub struct DateField {
    pub date: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExecutionPagination {
    pub executions: Vec<Execution>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Execution {
    pub id: i32,
    pub href: String,
    pub permalink: String,
    pub status: String,
    pub project: String,
    pub user: String,
    #[serde(rename = "date-started")]
    pub date: DateField,
    pub job: Job,
    pub description: String
}

pub fn kill(client: &Client, url: &str, token: &str, execution_id: &str) {

}
