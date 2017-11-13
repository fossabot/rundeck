use ResultExt;
use super::*;
use api::client::Client;
use api::JobService;
use api::ProjectService;
use dialoguer;
use api::job::RunBody;
use std::collections::HashMap;
use std::io::Write;

pub struct RunCommand {}

impl Processable for RunCommand {
    fn new() -> Self {
        Self {}
    }

    fn proceed(&mut self, matches: &ArgMatches, client: &Client) -> Result<()> {
        debug!("Entering RunCommand proceed function");
        let job_service =
            JobService::from_client(client).chain_err(|| "Cannot create a valid JobService")?;

        let project_service = ProjectService::from_client(client)
            .chain_err(|| "Cannot create a valid ProjectService")?;

        let job_id = if matches.value_of("job_id").is_some() {
            matches.value_of("job_id").unwrap().to_string()
        } else {
            let filters_raw: Vec<&str> = matches
                .values_of("filter")
                .map(|x| x.collect::<Vec<_>>())
                .unwrap_or_default();

            let mut filters: HashMap<String, Vec<String>> = HashMap::new();
            filters_raw.iter().for_each(|x| {
                let s: Vec<&str> = x.split('=').collect();
                let mut value = s[1..].iter().map(|x| x.to_string()).collect();
                let mut key = s[0].to_lowercase();

                match key.as_ref() {
                    "name" => {
                        key = "job".to_string();
                        value = format!("name={}", value);
                    }
                    "group" => {
                        key = "job".to_string();
                        value = format!("group={}", value);
                    }
                    _ => {}
                }

                let mut v = match filters.get(&key) {
                    Some(f) => f.clone(),
                    _ => Vec::new(),
                };

                v.append(&mut vec![value]);
                filters.insert(key, v);
            });

            trace!("Filters: {:?}", filters);

            trace!("Project's filter: {:?}", filters.get("project"));
            trace!("Job's filter: {:?}", filters.get("job"));

            let project: String = match matches.value_of("project") {
                Some(p) => p.into(),
                None => {
                    let pro: Vec<String> = project_service
                        .list()
                        .iter()
                        .cloned()
                        .map(|x| x.name.to_string())
                        .collect();
                    let pro_str: Vec<&str> = pro.iter().map(AsRef::as_ref).collect::<Vec<&str>>();

                    trace!("Projects: {:?}", pro_str);
                    let selection = dialoguer::Select::new()
                        .default(0)
                        .items(&pro_str[..])
                        .interact()
                        .unwrap();

                    pro_str[selection].to_string()
                }
            };

            trace!("Selected projects: {:?}", project);
            let jobs = job_service
                .list(
                    &project,
                    filters.get("job").cloned().unwrap_or_else(Vec::new),
                )
                .iter()
                .cloned()
                .map(|x| {
                    format!(
                        "{}{}{} ({})",
                        x.group.clone().unwrap_or_else(|| "".into()),
                        match x.group {
                            Some(_) => "/",
                            _ => "",
                        },
                        x.name,
                        x.id
                    )
                })
                .collect::<Vec<String>>();


            let job_str: Vec<&str> = jobs.iter().map(AsRef::as_ref).collect();

            trace!("Jobs founded: {:?}", jobs);

            if jobs.is_empty() {
                bail!("No jobs were found for this project.");
            }

            let selection = dialoguer::Select::new()
                .default(0)
                .items(&job_str[..])
                .interact()
                .unwrap();

            job_str[selection]
                .split(|c| c == '(' || c == ')')
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
                .pop()
                .unwrap()
                .to_string()
        };

        trace!("job_id: {:?}", job_id);

        let node: &str = matches.value_of("node").unwrap();

        trace!("node: {:?}", node);

        let options: Vec<&str> = matches
            .values_of("opt")
            .map(|x| x.collect::<Vec<_>>())
            .unwrap_or_default();

        trace!("options: {:?}", options);

        let mut body = RunBody {
            arg_string: None,
            filter: Some(node.into()),
            options: HashMap::new(),
        };

        for i in options {
            let split = i.split('=').collect::<Vec<_>>();

            if split.len() < 2 {
                bail!("Options must be key=value");
            }
            let name = split[0];
            let opt = split[1];

            body.options.insert(name.into(), opt.into());
        }

        trace!("Request body: {:?}", body);
        let permaink = job_service.run(&job_id, &body);

        info!("Successfuly launch the job.\n");
        println!("{}", permaink);
        let _ = ::std::io::stdout().flush();

        Ok(())
    }
}
