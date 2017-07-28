use reqwest::Client;
use serde_json;

use prettytable::format;
use prettytable::row::Row;
use prettytable::cell::Cell;
use std::io::Read;
use super::construct_headers;
use super::ExecutionPagination;

#[derive(Clone, Debug, Deserialize)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub group: String,
    pub project: String,
    pub href: String,
    pub permalink: String,
    pub description: String,
    pub schedule_enabled: Option<bool>,
    pub enabled: Option<bool>,
    pub scheduled: Option<bool>,
}

pub fn list_jobs(client: &Client, url: &str, token: &str, project: &str, quiet: bool) {
    let mut res = client.get(&format!("{}project/{}/jobs?authtoken={}", url, project, token )).unwrap()
        .headers(construct_headers())
        .send().unwrap();

    let mut content = String::new();
    let _ = res.read_to_string(&mut content);

    let jobs: Vec<Job> = serde_json::from_str(&content).unwrap();

    if !quiet {
        let mut table = table!(["ID", "NAME", "DESCRIPTION"]);

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        for j in jobs {
            table.add_row(Row::new(vec![
                                   Cell::new(&j.id),
                                   Cell::new(&j.name),
                                   Cell::new(&j.description)]));
        }

        table.printstd();
    } else {
        for j in jobs {
            println!("{}", j.id);
        }
    }
}

pub fn list_job_executions(client: &Client, url: &str, token: &str, job: &str) {
    let mut res = client.get(&format!("{}job/{}/executions?authtoken={}", url, job, token )).unwrap()
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
