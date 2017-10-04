extern crate assert_cli;
use std::process::Command;

static HELP_ALL: &'static str = "Rundeck CLI 1.0
Simon PAITRAULT <simon.paitrault@gmail.com>

The Rundeck Command Line Interface is a tool to manage, run and display jobs and projects.
It use the Rundeck API, you must define a RUNDECK_URL and a RUNDECK_TOKEN.

USAGE:
    Rundeck CLI [SUBCOMMAND]

SUBCOMMANDS:
    auth    Authenticate with username/password (You should use this to generate a token and then use the token)
    kill    Kill a job
    list    List projects, job, executions
    new     Create new token, job, ...
    run     Run a particular job
";

#[cfg(test)]
mod integration {
    use assert_cli;
    use HELP_ALL;

    #[test]
    fn calling_rundeck_without_args() {
        assert_cli::Assert::main_binary()
            .stdout().is(HELP_ALL)
            .unwrap();
        // let output = Command::new("../target/debug/rundeck")
        //     .output()
        //     .expect("failed to execute process");

        // assert_eq!(String::from_utf8_lossy(&output.stdout), HELP_ALL);
    }
}
