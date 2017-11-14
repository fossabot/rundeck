static HELP_ALL: &str = "Rundeck CLI 1.0
Simon PAITRAULT <simon.paitrault@gmail.com>

The Rundeck Command Line Interface is a tool to manage, run and display \
jobs and projects.
It use the Rundeck API, you must define a RUNDECK_URL and a RUNDECK_TOKEN.

USAGE:
    Rundeck CLI [FLAGS] [SUBCOMMAND]

FLAGS:
    -q        Only display important informations
    -v        Sets the level of verbosity

SUBCOMMANDS:
    auth        Authenticate with username/password \
    (You should use this to generate a token and then use the token)
    jobs        List and manage jobs.
    projects    List and manage projects.
    run         Run a particular job
    tokens      List and manage tokens.
";

#[cfg(test)]
mod integration {
    extern crate mockito;
    use HELP_ALL;
    use std::process::Command;

    #[test]
    fn calling_rundeck_without_args() {
        let _m = mockito::mock("GET", "/20/system/info")
            .match_header("Accept", mockito::Matcher::Any)
            .match_header("X-Rundeck-Auth-Token", mockito::Matcher::Any)
            .with_status(200)
            .create();

        let mut args: Vec<String> = vec!["run".into()];

        println!("{:?}", ::std::env::var("TARGET"));
        if let Ok(t) = ::std::env::var("TARGET") {
            args.push("--target".into());
            args.push(t.clone());
        }

        let output = Command::new("cargo")
            .args(&args)
            .env("RUNDECK_URL", format!("{}/20/", mockito::SERVER_URL))
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), HELP_ALL);
        mockito::reset();
    }
}
