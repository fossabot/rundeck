use ResultExt;
use super::*;
use super::Processable;
use prettytable::format;
use prettytable::row::Row;
use prettytable::cell::Cell;
use api::Project;
use api::ProjectService;


pub struct ListProjectsCommand {}

impl Processable for ListProjectsCommand {
    fn new() -> Self {
        Self {}
    }

    fn proceed<'a>(&mut self, matches: &ArgMatches, client: &Client<'a>) -> Result<()> {
        let project_service = ProjectService::from_client(client)
            .chain_err(|| "Cannot create a valid ProjectService")?;

        let projects: Vec<Project> = project_service.list();

        if matches.is_present("quiet") {
            for p in projects {
                println!("{}", p.name);
            }
        } else {
            let mut table = table!(["NAME", "DESCRIPTION"]);

            table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
            for p in projects {
                table.add_row(Row::new(
                    vec![Cell::new(&p.name), Cell::new(&p.description.unwrap())],
                ));
            }

            table.printstd();
        }
        Ok(())
    }
}
