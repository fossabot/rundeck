static RUN_STARTED: &str = "Successfuly launch the job.

http://url/job/1
";

#[cfg(test)]
mod integration_run {
    extern crate assert_cli;
    extern crate mockito;

    use std::process::Command;

    use RUN_STARTED;

    #[test]
    fn calling_rundeck_run() {
        let _m = mockito::mock("GET", "/20/system/info")
            .match_header("Accept", mockito::Matcher::Any)
            .match_header("X-Rundeck-Auth-Token", mockito::Matcher::Any)
            .with_status(200)
            .create();

        let _z = mockito::mock("POST", "/20/job/id/run?format=json")
            .match_header("Accept", mockito::Matcher::Any)
            .match_header("Content-Type", "application/json")
            .match_header("X-Rundeck-Auth-Token", mockito::Matcher::Any)
            .with_status(201)
            .with_body("{\"permalink\":\"http://url/job/1\"}")
            .create();

        let mut args: Vec<String> = vec!["run".into()];

        if let Ok(t) = ::std::env::var("TARGET") {
            args.push("--target".into());
            args.push(t.clone());
        }

        let mut cmd: Vec<String> = ["--", "run", "--job-id", "id", "--node", "localhost"]
            .iter_mut()
            .map(|x| x.to_string())
            .collect();

        args.append(&mut cmd);

        let output = Command::new("cargo")
            .env("RUNDECK_URL", format!("{}/20/", mockito::SERVER_URL))
            .env("RUNDECK_TOKEN", "token")
            .args(&args)
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), RUN_STARTED);

        mockito::reset();
    }
}
