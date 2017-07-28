//!
//! rundeck list projects
//! rundeck list jobs <project>
//! rundeck list executions job <job_id>
//! rundeck list executions project <project>
//! rundeck run job <job>
//! rundeck kill job <job_id>
//!
#[macro_use]extern crate clap;
extern crate reqwest;
#[macro_use] extern crate prettytable;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::env;
use clap::App;
use reqwest::header::{Headers, ContentType, Accept};

mod project;
mod job;
mod execution;

use job::Job;
use execution::{ExecutionPagination};

pub fn construct_headers() -> Headers {
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set(Accept::json());
    headers
}


fn main() {
    let url = env::var("RUNDECK_URL").expect("RUNDECK_URL NOT DEFINED");
    let authtoken = env::var("RUNDECK_TOKEN").expect("RUNDECK_TOKEN NOT DEFINED");

    let mut help_bytes: Vec<u8> = Vec::new();
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);

    app.write_help(&mut help_bytes).expect("Failed to capture help message");

    let matches = app.get_matches();

    let client = reqwest::Client::new().unwrap();

    match matches.subcommand() {
        ("list", Some(list_matches)) =>{
            match list_matches.subcommand() {

                ("projects", _) =>
                    project::list_projects(&client, &url, &authtoken),

                ("jobs", Some(matches)) =>
                    job::list_jobs(&client, &url, &authtoken, matches.value_of("project").unwrap(), matches.is_present("quiet")),

                ("executions", Some(executions_matches)) => {
                    match executions_matches.subcommand() {

                        ("project", Some(matches)) =>
                            project::list_project_executions(&client, &url, &authtoken, matches.value_of("project").unwrap()),

                        ("job", Some(matches)) =>
                            job::list_job_executions(&client, &url, &authtoken, matches.value_of("job_id").unwrap()),

                        _ =>
                            unreachable!()
                    }

                }
                _ => unreachable!()
            }
        },
        ("run", Some(projects_matches)) =>
            match projects_matches.subcommand() {

                ("job", Some(matches)) =>{

                    let job = matches.value_of("job_id").unwrap();

                    println!("{:?}", job);
                },
                ("list", Some(_)) =>{
                },
                _            => unreachable!(),
            },

        ("kill", Some(matches)) =>
            execution::kill(&client, &url, &authtoken, &matches.value_of("execution_id").unwrap()),

        ("", None) =>
            println!("{}", String::from_utf8(help_bytes).expect("Help message was invalid UTF8")),

        _ =>
            unreachable!(),
    }
}
