use serde_json;
use prettytable::format;
use prettytable::row::Row;
use prettytable::cell::Cell;

use super::ExecutionPagination;
use super::construct_headers;
use std::io::Read;
use reqwest::Client;

#[derive(Clone, Debug, Deserialize)]
pub struct Project {
    pub url: String,
    pub description: Option<String>,
    pub name: String
}

pub fn list_projects(client: &Client, url: &str, token: &str) {
    let mut res = client.get(&format!("{}projects?authtoken={}", url, token )).unwrap()
        .headers(construct_headers())
        .send().unwrap();

    let mut content = String::new();
    let _ = res.read_to_string(&mut content);

    let projects: Vec<Project> = serde_json::from_str(&content).unwrap();
    let mut table = table!(["NAME", "DESCRIPTION"]);

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    for p in projects {
        table.add_row(Row::new(vec![
                               Cell::new(&p.name),
                               Cell::new(&p.description.unwrap())]));
    }

    table.printstd();
}

pub fn list_project_executions(client: &Client, url: &str, token: &str, project: &str) {
    let mut res = client.get(&format!("{}project/{}/executions?authtoken={}", url, project, token )).unwrap()
        .headers(construct_headers())
        .send().unwrap();

    let mut content = String::new();
    let _ = res.read_to_string(&mut content);

    let executions: ExecutionPagination = serde_json::from_str(&content).unwrap();
    let mut table = table!(["ID", "PROJECT", "GROUP", "NAME", "STATUS", "STARTED BY", "LINK"]);

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    for e in executions.executions {
        table.add_row(Row::new(vec![
                               Cell::new(&e.id.to_string()),
                               Cell::new(&e.project),
                               Cell::new(&e.job.group),
                               Cell::new(&e.job.name),
                               Cell::new(&e.status),
                               Cell::new(&e.user),
                               Cell::new(&e.permalink)
        ]));
    }

    table.printstd();
}
