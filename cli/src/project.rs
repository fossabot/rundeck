use prettytable::format;
use prettytable::row::Row;
use prettytable::cell::Cell;
use api::Project;
use api::ProjectService;

pub fn list_projects(service: &ProjectService, quiet: bool) {
    let projects: Vec<Project> = service.list();

    if quiet {
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
}
