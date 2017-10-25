use ResultExt;
use dialoguer;
use std::env;
use super::*;
use super::Processable;
use api::TokenService;
use api::Token;
use api::token::TokenBody;
use std::borrow::Cow;

pub struct AuthCommand {}

impl Processable for AuthCommand {
    fn new() -> Self {
        Self {}
    }

    fn proceed<'a>(&mut self, matches: &ArgMatches, client: &Client<'a>) -> Result<()> {
        // Check if Token
        if let Ok(t) = env::var("RUNDECK_TOKEN") {
            // If Token
            //  -> Check if valid
            // if client.check_connectivity().is_err() {
                if let Err(e) = client.check_connectivity() {
                    println!("{:?}", e);
                //  -> If Not
                //      -> Log with user:password
                info!("It seems that you already have a RUNDECK_TOKEN");
                info!("Checking if not expired...");
                info!("Your RUNDECK_TOKEN is expired or invalid");
                info!("We will fetch or create a new one.");

                let (username, password) = self.ask_username_password(matches)?;

                self.display_token(client, username, password);
            } else {
                info!("Your token is valid\n");
                println!("{}", t);
                info!("\n     export RUNDECK_TOKEN={}", t);
            }
        } else {
            info!("Your RUNDECK_TOKEN is missing");
            info!("We will fetch or create a new one.");
            let (username, password) = self.ask_username_password(matches)?;

            self.display_token(client, username, password);
        }

        Ok(())
    }
}

impl AuthCommand {
    fn display_token(&self, client: &Client, username: String, password: String) {
        match self.fetch_or_create_token(client, username, password) {
            Ok(t) => {
                info!("Here's your token: {}", t);
                info!("Use this token with:");
                info!("\n     export RUNDECK_TOKEN=");
                info!("{}", t);
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }
    }

    fn fetch_or_create_token(
        &self,
        client: &Client,
        username: String,
        password: String,
    ) -> Result<String> {
        let mut rundeck = client.clone();


        rundeck.erase_token();
        rundeck
            .auth(username.clone(), password)
            .chain_err(|| "Fail to auth")?;

        rundeck.check_connectivity().chain_err(|| "Fail")?;

        let s =
            TokenService::from_client(&rundeck).chain_err(|| "Cannot create a valid TokenService")?;

        let token_list = s.list(Vec::new());

        let x: Vec<&Token> = token_list
            .iter()
            .filter(|x| x.creator == username)
            .filter(|x| x.expired)
            .collect();

        let v: Vec<_> = x.into_iter().map(|x| s.get(x).unwrap()).collect();

        if !v.is_empty() {
            match v[0].token {
                Some(ref t) => Ok(t.to_string()),
                None => self.create_token(&s, &username),
            }
        } else {
            self.create_token(&s, &username)
        }
    }

    fn create_token(&self, service: &TokenService, user: &str) -> Result<String> {
        let body = TokenBody {
            user: Cow::from(user),
            roles: vec![
                Cow::from("user"),
                Cow::from("deploy"),
                Cow::from("build"),
                Cow::from("architect"),
                Cow::from("admin"),
            ],
            duration: Cow::from("30d"),
        };

        match service.create(&body) {
            Ok(t) => {
                let t = match t.token {
                    Some(t) => t.to_string(),
                    None => {
                        println!("No token");
                        bail!("fail")
                    }
                };

                Ok(t)
            }
            Err(e) => bail!("fail {:#?}", e),
        }
    }

    fn ask_username_password(&self, matches: &ArgMatches) -> Result<(String, String)> {
        Ok((
            match matches.value_of("username") {
                Some(u) => u.to_string(),
                None => dialoguer::Input::new("username")
                    .interact()
                    .chain_err(|| "You need to provide a username")?,
            },
            match matches.value_of("password") {
                Some(u) => u.to_string(),
                None => dialoguer::PasswordInput::new("password")
                    .interact()
                    .chain_err(|| "You need to provide a password")?,
            },
        ))
    }
}
