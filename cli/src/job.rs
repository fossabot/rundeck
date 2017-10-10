use api::job::JobService;
use api::Job;
use api::job::RunBody;
use prettytable::format;
use prettytable::row::Row;
use prettytable::cell::Cell;
use std::collections::HashMap;

pub fn list_jobs(service: &JobService, project: &str, quiet: bool, completion: bool, filters: Vec<&str>) {
    let jobs: Vec<Job> = service.list(project, filters);

    if quiet {
        for j in jobs {
            if completion {
                println!("{}/{}({})", j.group.unwrap_or_else(||"".into()), j.name, j.id);
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
                                   Cell::new(&j.description)]));
        }

        table.printstd();
    }
}

pub fn run(service: &JobService, job_id: &str, node: &str, options: Vec<&str>) {
    // Options to RunBody
    println!("start cli run");
    let mut body = RunBody {arg_string: None, filter: Some(node.into()), options: HashMap::new()};
    for i in options {
        let split = i.split('=').collect::<Vec<_>>();

        let name = split[0];
        let opt = split[1];

        body.options.insert(name.into(), opt.into());
    }

    service.run(job_id, &body);
}

// pub fn list_job_executions(service: &JobService, job: &str) {
//     let mut res = client.get(&format!("{}job/{}/executions?authtoken={}", url, job, token )).unwrap()
//         .headers(construct_headers())
//         .send().unwrap();

//     let mut content = String::new();
//     let _ = res.read_to_string(&mut content);

//     let executions: ExecutionPagination = serde_json::from_str(&content).unwrap();
//     let mut table = table!(["ID", "PROJECT", "GROUP", "NAME", "STATUS", "STARTED BY", "LINK"]);

//     table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
//     for e in executions.executions {
//         table.add_row(Row::new(vec![
//                                Cell::new(&e.id.to_string()),
//                                Cell::new(&e.project),
//                                Cell::new(&e.job.group),
//                                Cell::new(&e.job.name),
//                                Cell::new(&e.status),
//                                Cell::new(&e.user),
//                                Cell::new(&e.permalink)
//         ]));
//     }

//     table.printstd();
// }

