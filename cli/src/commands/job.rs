use ResultExt;
use super::*;
use api::client::Client;
use api::ProjectService;
use api::JobService;
use dialoguer;
use api::Job;
use prettytable::format;
use prettytable::row::Row;
use prettytable::cell::Cell;

pub struct ListJobsCommand {}

impl Processable for ListJobsCommand {
    fn new() -> Self {
        Self {}
    }
    fn proceed<'a>(&mut self, matches: &ArgMatches, client: &Client<'a>) -> Result<()> {
        let job_service =
            JobService::from_client(client).chain_err(|| "Cannot create a valid JobService")?;

        let project = {
            if matches.value_of("project").is_none() {
                let project_service = ProjectService::from_client(client)
                    .chain_err(|| "Cannot create a valid ProjectService")?;
                let jobs: Vec<String> = project_service
                    .list()
                    .iter()
                    .map(|x| format!("{}", x.name))
                    .collect();

                let job_str: Vec<&str> = jobs.iter().map(AsRef::as_ref).collect();

                let selection = dialoguer::Select::new()
                    .default(0)
                    .items(&job_str[..])
                    .interact()
                    .unwrap();

                job_str[selection].to_string()
            } else {
                matches.value_of("project").unwrap_or("*").to_string()
            }
        };

        let filters: Vec<&str> = matches
            .values_of("filter")
            .map(|x| x.collect::<Vec<_>>())
            .unwrap_or_default();

        let jobs: Vec<Job> = job_service.list(&project, filters);

        if matches.is_present("quiet") {
            for j in jobs {
                if matches.is_present("completion") {
                    println!(
                        "{}/{}({})",
                        j.group.unwrap_or_else(|| "".into()),
                        j.name,
                        j.id
                    );
                } else {
                    println!("{}", j.id);
                }
            }
        } else {
            let mut table = table!(["ID", "GROUP/NAME", "DESCRIPTION"]);

            table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

            for j in jobs {
                table.add_row(Row::new(vec![
                    Cell::new(&j.id),
                    Cell::new(&j.name_with_group()),
                    Cell::new(&j.description),
                ]));
            }

            table.printstd();
        }

        Ok(())
    }
}
