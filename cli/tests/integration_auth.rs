// static HELP_ALL: &'static str = "Rundeck CLI 1.0
// Simon PAITRAULT <simon.paitrault@gmail.com>

// The Rundeck Command Line Interface is a tool to manage, run and display jobs and projects.
// It use the Rundeck API, you must define a RUNDECK_URL and a RUNDECK_TOKEN.

// USAGE:
//     Rundeck CLI [SUBCOMMAND]

// SUBCOMMANDS:
//     auth    Authenticate with username/password (You should use this to generate a token \
//     and then use the token)
//     kill    Kill a job
//     list    List projects, job, executions
//     new     Create new token, job, ...
//     run     Run a particular job
// ";

static TOKEN_OK: &'static str = "Your token is valid

     export RUNDECK_TOKEN=token
";
#[cfg(test)]
mod integration_auth {
    extern crate mockito;
    use std::process::Command;
    // use HELP_ALL;
    use TOKEN_OK;

    #[test]
    #[ignore]
    fn calling_rundeck_auth_with_user_pass() {
        let _m = mockito::mock("GET", "/20/system/info")
            .match_header("Accept", mockito::Matcher::Any)
            .match_header("X-Rundeck-Auth-Token", mockito::Matcher::Any)
            .with_status(200)
            .create();

        let output = Command::new("cargo")
            .env("RUNDECK_URL", format!("{}/20/", mockito::SERVER_URL))
            .env("RUNDECK_TOKEN", "token")
            .args(&["run", "--", "auth", "-u", "user", "-p", "pass"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), TOKEN_OK);

        mockito::reset();
    }

    #[test]
    #[ignore]
    fn calling_rundeck_auth_with_user_pass_quiet() {
        let _m = mockito::mock("GET", "/20/system/info")
            .match_header("Accept", mockito::Matcher::Any)
            .match_header("X-Rundeck-Auth-Token", mockito::Matcher::Any)
            .with_status(200)
            .create();

        let output = Command::new("cargo")
            .env("RUNDECK_URL", format!("{}/20/", mockito::SERVER_URL))
            .env("RUNDECK_TOKEN", "token")
            .args(&["run", "--", "auth", "-q", "-u", "user", "-p", "pass"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "token");

        mockito::reset();
    }
}
