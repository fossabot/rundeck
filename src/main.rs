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
extern crate dialoguer;

use std::env;
use clap::App;
use reqwest::header::{Headers, ContentType, Accept};

mod job;
mod api;
mod project;
mod execution;

pub fn construct_headers() -> Headers {
    let mut headers = Headers::new();
    headers.set(Accept::json());
    headers
}


fn main() {
    let url = env::var("RUNDECK_URL").expect("RUNDECK_URL NOT DEFINED");
    let authtoken = env::var("RUNDECK_TOKEN").expect("RUNDECK_TOKEN NOT DEFINED");

    let rundeck = api::Client::new(url, authtoken);

    if let Err(e) = rundeck.check_connectivity() {
        println!("Rundeck is not accessible on HTTP/HTTPs protocol.");
        std::process::exit(1);
    }

    let job_service = api::JobService::from_client(&rundeck).expect("Cannot create a valid JobService");
    let project_service = api::ProjectService::from_client(&rundeck).expect("Cannot create a valid ProjectService");

    let mut help_bytes: Vec<u8> = Vec::new();
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);

    app.write_help(&mut help_bytes).expect("Failed to capture help message");

    let matches = app.get_matches();

    match matches.subcommand() {
        ("list", Some(list_matches)) =>{
            match list_matches.subcommand() {

                ("projects", Some(matches)) =>
                    project::list_projects(&project_service, matches.is_present("quiet")),

                    ("jobs", Some(matches)) =>
                    {
                        let project = {
                            if matches.value_of("project").is_none() {
                                let jobs: Vec<String> = project_service.list()
                                    .iter()
                                    .map(|x| format!("{}",  x.name))
                                    .collect();

                                let job_str: Vec<&str> = jobs.iter()
                                    .map(AsRef::as_ref)
                                    .collect();

                                let selection = dialoguer::Select::new()
                                    .default(0)
                                    .items(&job_str[..])
                                    .interact().unwrap();

                                job_str[selection].to_string()
                            } else {
                                matches.value_of("project").unwrap_or("*").to_string()
                            }
                        };

                        job::list_jobs(&job_service,
                                       &project,
                                       matches.is_present("quiet"),
                                       matches.is_present("completion"),
                                       matches.values_of("filter")
                                                .map(|x| x.collect::<Vec<_>>())
                                                .unwrap_or(Vec::new()))
                },

                ("executions", Some(executions_matches)) => {
                    match executions_matches.subcommand() {

                        ("project", Some(_)) => {}
                            // project::list_project_executions(&client, &url, &authtoken, matches.value_of("project").unwrap()),

                        ("job", Some(matches)) =>{}
                            // job::list_job_executions(&job_service, matches.value_of("job_id").unwrap()),

                        _ =>
                            println!("{}", String::from_utf8(help_bytes).expect("Help message was invalid UTF8")),
                    }

                }
                _ =>
                    println!("{}", String::from_utf8(help_bytes).expect("Help message was invalid UTF8")),
            }
        },
        ("run", Some(matches)) => {
            let mut job_id = String::new();
            if matches.value_of("job_id").is_some() {
                println!("A job id");
                job_id = matches.value_of("job_id").unwrap().to_string();
            } else {
                let jobs: Vec<String> = job_service.list(matches.value_of("project").unwrap(), matches.values_of("filter")
                                                .map(|x| x.collect::<Vec<_>>())
                                                .unwrap_or(Vec::new()))
                    .iter()
                    .cloned()
                    .map(|x| format!("{}/{} ({})", x.group.unwrap_or(String::new()), x.name, x.id))
                    .collect();

                let job_str: Vec<&str> = jobs.iter()
                    .map(AsRef::as_ref)
                    .collect();

                let selection = dialoguer::Select::new()
                    .default(0)
                    .items(&job_str[..])
                    .interact().unwrap();

                job_id = job_str[selection].split(|c| c == '(' || c == ')').filter(|x| x.len() > 0).collect::<Vec<_>>().pop().unwrap().to_string();
            }

            job::run(&job_service, &job_id, matches.value_of("node").unwrap(), matches.values_of("opt").map(|x|x.collect::<Vec<_>>()).unwrap());
        },

        ("kill", Some(_)) =>{}
            // execution::kill(&client, &url, &authtoken, &matches.value_of("execution_id").unwrap()),

        ("", None) =>
            println!("{}", String::from_utf8(help_bytes).expect("Help message was invalid UTF8")),

        _ =>
            unreachable!(),
    }
}