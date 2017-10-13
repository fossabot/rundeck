use ResultExt;
use super::*;
use api::client::Client;
use api::JobService;
use dialoguer;
use api::job::RunBody;
use std::collections::HashMap;

pub struct RunCommand {}

impl Processable for RunCommand {
    fn new() -> Self {
        Self {}
    }

    fn proceed(&mut self, matches: &ArgMatches, client: &Client) -> Result<()> {
        let job_service =
            JobService::from_client(client).chain_err(|| "Cannot create a valid JobService")?;

        let job_id = if matches.value_of("job_id").is_some() {
            matches.value_of("job_id").unwrap().to_string()
        } else {
            let jobs: Vec<String> = job_service
                .list(
                    matches.value_of("project").unwrap(),
                    matches
                        .values_of("filter")
                        .map(|x| x.collect::<Vec<_>>())
                        .unwrap_or_default(),
                )
                .iter()
                .cloned()
                .map(|x| {
                    format!(
                        "{}/{} ({})",
                        x.group.unwrap_or_else(|| "".into()),
                        x.name,
                        x.id
                    )
                })
                .collect();

            let job_str: Vec<&str> = jobs.iter().map(AsRef::as_ref).collect();

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

        let node: &str = matches.value_of("node").unwrap();
        let options: Vec<&str> = matches
            .values_of("opt")
            .map(|x| x.collect::<Vec<_>>())
            .unwrap_or_default();

        let mut body = RunBody {
            arg_string: None,
            filter: Some(node.into()),
            options: HashMap::new(),
        };
        for i in options {
            let split = i.split('=').collect::<Vec<_>>();

            let name = split[0];
            let opt = split[1];

            body.options.insert(name.into(), opt.into());
        }

        job_service.run(&job_id, &body);

        Ok(())
    }
}
