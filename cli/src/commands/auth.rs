use ResultExt;
use dialoguer;
use std::env;
use super::*;
use super::Command;
use clap::ArgMatches;
use api::client::Client;
use api::TokenService;
use api::Token;

pub struct AuthCommand<'a> {
    matches: &'a ArgMatches<'a>,
    client: &'a Client<'a>,
}

impl<'a> Command for AuthCommand<'a> {
    fn proceed(&mut self) -> Result<()> {
        // Check if Token
        if let Ok(t) = env::var("RUNDECK_TOKEN") {
            // If Token
            //  -> Check if valid
            if let Err(_) = self.client.check_connectivity() {
                //  -> If Not
                //      -> Log with user:password
                self.println("It seems that you already have a RUNDECK_TOKEN");
                self.println("Checking if not expired...");
                self.println("Your RUNDECK_TOKEN is expired or invalid");
                self.println("We will fetch or create a new one.");
                let (username, password) = self.ask_username_password()?;

                self.display_token(username, password);

            } else {
                self.println("Your token is valid");
                self.print("\n     export RUNDECK_TOKEN=");
                print!("{}", t);
                self.println("");
            }
        } else {
            self.print("Your RUNDECK_TOKEN is missing");
            self.print("We will fetch or create a new one.");
            let (username, password) = self.ask_username_password()?;

            self.display_token(username, password);
        }

        Ok(())
    }

    fn is_quiet(&self) -> bool {
        self.matches.is_present("quiet")
    }
}

impl<'a> AuthCommand<'a> {

    fn display_token(&self, username: String, password: String) {
        match self.fetch_or_create_token(username, password) {
            Ok(t) => {
                self.println(format!("Here's your token: {}", t));
                self.println("Use this token with:");
                self.print("\n     export RUNDECK_TOKEN=");
                print!("{}", t);
                self.println("");
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    fn fetch_or_create_token(&self, username: String, password: String) -> Result<String> {
        let mut rundeck = self.client.clone();
        rundeck.erase_token();
        rundeck.auth(username.clone(), password).chain_err(|| "Fail to auth")?;
        rundeck.check_connectivity().chain_err(|| "Fail")?;
        let s = TokenService::from_client(&rundeck).chain_err(|| "Cannot create a valid TokenService")?;

        let token_list = s.list(Vec::new());

        let x: Vec<&Token> = token_list.iter()
            .filter(|x| {
                x.creator == username
            })
            .filter(|x| {
                !x.expired
            })
            .collect();

        let v:Vec<_> = x.into_iter().map(|x| {
            s.get(x).unwrap()
        })
        .collect();

        if !v.is_empty() {
            match v[0].token {
                Some(ref t) => {
                    Ok(t.to_string())
                },
                None => {
                    bail!("No token")
                }
            }
        } else {
            bail!("No token")
        }
    }

    fn ask_username_password(&self) -> Result<(String, String)> {
        Ok((
            match self.matches.value_of("username") {
                Some(u) => u.to_string(),
                None => dialoguer::Input::new("username").interact().chain_err(|| "You need to provide a username")?
            },
            match self.matches.value_of("password") {
                Some(u) => u.to_string(),
                None => dialoguer::PasswordInput::new("password").interact().chain_err(|| "You need to provide a password")?
            }
        ))
    }

    pub fn from_matches(matches: &'a ArgMatches, client: &'a Client) -> Self {
        Self {
            matches,
            client
        }
    }
}
