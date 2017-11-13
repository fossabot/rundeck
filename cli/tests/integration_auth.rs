static TOKEN_OK_QUIET: &str = "token";
static TOKEN_OK: &'static str = "You already have a valid RUNDECK_TOKEN.
Your token is:

token

You can use this export command to add it now to your env:

    export RUNDECK_TOKEN=token

Or even better, to your shell profile:

    echo 'export RUNDECK_TOKEN=token' >> ~/.profile
";
#[cfg(test)]
mod integration_auth {
    extern crate assert_cli;
    extern crate environment;
    extern crate mockito;

    use std::process::Command;

    use TOKEN_OK;
    use TOKEN_OK_QUIET;

    #[test]
    fn calling_rundeck_auth_with_user_pass() {
        let _m = mockito::mock("GET", "/20/system/info")
            .match_header("Accept", mockito::Matcher::Any)
            .match_header("X-Rundeck-Auth-Token", mockito::Matcher::Any)
            .with_status(200)
            .create();

        let mut args: Vec<String> = vec!["run".into()];

        if let Ok(t) = ::std::env::var("TARGET") {
            args.push("--target".into());
            args.push(t.clone());
        }

        let mut cmd: Vec<String> = ["--", "auth", "-u", "user", "-p", "pass"]
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

        assert_eq!(String::from_utf8_lossy(&output.stdout), TOKEN_OK);

        mockito::reset();
    }

    #[test]
    fn calling_rundeck_auth_with_user_pass_quiet() {
        let _m = mockito::mock("GET", "/20/system/info")
            .match_header("Accept", mockito::Matcher::Any)
            .match_header("X-Rundeck-Auth-Token", mockito::Matcher::Any)
            .with_status(200)
            .create();

        let mut args: Vec<String> = vec!["run".into()];

        if let Ok(t) = ::std::env::var("TARGET") {
            args.push("--target".into());
            args.push(t.clone());
        }

        let mut cmd: Vec<String> = ["--", "auth", "-q", "-u", "user", "-p", "pass"]
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

        assert_eq!(String::from_utf8_lossy(&output.stdout), TOKEN_OK_QUIET);

        mockito::reset();
    }
}
