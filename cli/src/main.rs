//!
//! rundeck list projects
//! rundeck list jobs <project>
//! rundeck list executions job <`job_id`>
//! rundeck list executions project <project>
//! rundeck run job <job>
//! rundeck kill job <`job_id`>
//!
#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
extern crate dialoguer;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate prettytable;
extern crate reqwest;
extern crate rundeck_api as api;
extern crate serde;
extern crate serde_json;

extern crate fern;
#[macro_use]
extern crate log;

use std::env;
use clap::App;
use reqwest::header::{Accept, Headers};

error_chain!{
    foreign_links {
        Api(api::error::ClientError);
    }

    errors {
        NoCommandProvided(h: String) {
            description("No command provided")
            display("{}", h)
        }
    }
}

mod commands;

use commands::Command;

pub fn construct_headers() -> Headers {
    let mut headers = Headers::new();
    headers.set(Accept::json());
    headers
}

fn main() {
    if let Err(ref e) = start() {
        println!("{}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }
    }
}

fn start() -> Result<()> {
    let url = env::var("RUNDECK_URL").chain_err(|| "RUNDECK_URL NOT DEFINED")?;
    let authtoken = env::var("RUNDECK_TOKEN").unwrap_or_else(|_| "".to_string());

    let rundeck =
        api::client::Client::new(url, authtoken).chain_err(|| "Fail to create an api client")?;

    let mut help_bytes: Vec<u8> = Vec::new();
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);

    app.write_help(&mut help_bytes)
        .expect("Failed to capture help message");

    let matches = app.get_matches();

    let loglevel = if matches.is_present("quiet") {
        log::LogLevelFilter::Error
    } else {
        match matches.occurrences_of("verbose") {
            0 => log::LogLevelFilter::Info,
            1 => log::LogLevelFilter::Debug,
            2 | _ => log::LogLevelFilter::Trace,
        }
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            if record.level() == log::LogLevel::Info {
                out.finish(format_args!("{}", message))
            } else {
                out.finish(format_args!(
                    "[{}][{}]{}{}",
                    record.target(),
                    record.level(),
                    match record.level() {
                        log::LogLevel::Trace => " --> ",
                        _ => " ",
                    },
                    message
                ))
            }
        })
        .level(log::LogLevelFilter::Off)
        .level_for("rundeck", loglevel)
        .level_for("api", loglevel)
        .chain(std::io::stdout())
        .apply()
        .expect("Fail to create a valid stdout");

    match matches.subcommand() {
        ("auth", Some(auth_matches)) => {
            debug!("Auth command detected");
            Command::<commands::AuthCommand>::from_matches(auth_matches, &rundeck).proceed()?
        }

        ("projects", Some(matches)) => match matches.subcommand() {
            ("list", Some(matches)) => {
                debug!("Projects.list command detected");
                Command::<commands::ListProjectsCommand>::from_matches(matches, &rundeck).proceed()?
            }
            _ => bail!(ErrorKind::NoCommandProvided(matches.usage().into())),
        },

        ("jobs", Some(matches)) => match matches.subcommand() {
            ("list", Some(matches)) => {
                debug!("Jobs.list command detected");
                Command::<commands::ListJobsCommand>::from_matches(matches, &rundeck).proceed()?
            }
            _ => bail!(ErrorKind::NoCommandProvided(matches.usage().into())),
        },

        ("tokens", Some(matches)) => match matches.subcommand() {
            ("list", Some(matches)) => {
                debug!("Tokens.list command detected");
                Command::<commands::ListTokensCommand>::from_matches(matches, &rundeck).proceed()?
            }
            _ => bail!(ErrorKind::NoCommandProvided(matches.usage().into())),
        },

        ("run", Some(matches)) => {
            debug!("Run command detected");
            Command::<commands::RunCommand>::from_matches(matches, &rundeck).proceed()?
        }

        ("kill", Some(matches)) => bail!(ErrorKind::NoCommandProvided(matches.usage().into())),

        ("new", Some(matches)) => match matches.subcommand() {
            ("token", Some(matches)) => Command::<commands::TokenCreationCommand>::from_matches(
                matches,
                &rundeck,
            ).proceed()?,
            _ => bail!(ErrorKind::NoCommandProvided(matches.usage().into())),
        },

        ("", None) | _ => bail!(ErrorKind::NoCommandProvided(
            String::from_utf8(help_bytes).unwrap()
        )),
    }

    Ok(())
}
