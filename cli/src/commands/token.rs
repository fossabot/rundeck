use ResultExt;
use super::*;
use api::client::Client;
use api::TokenService;
use api::token::TokenBody;
use prettytable::format;
use prettytable::row::Row;
use prettytable::cell::Cell;
use api::Token;

pub struct TokenCreationCommand {}

impl Processable for TokenCreationCommand {
    fn new() -> Self {
        Self {}
    }

    fn proceed<'a>(&mut self, matches: &ArgMatches, client: &Client<'a>) -> Result<()> {
        let token_service =
            TokenService::from_client(client).chain_err(|| "Cannot create a valid TokenService")?;

        let user: &str = matches.value_of("user").expect("Fail");
        let duration: Option<&str> = matches.value_of("duration");
        let roles: Vec<&str> = matches
            .values_of("role")
            .map(|x| x.collect::<Vec<_>>())
            .unwrap_or_default();

        let body = TokenBody {
            user: user.into(),
            roles: roles.into_iter().map(|x| x.into()).collect::<_>(),
            duration: duration.unwrap_or("").into(),
        };

        let token = token_service.create(&body);

        match token {
            Ok(t) => println!("{}", t.id),
            Err(e) => println!("Error: {}", e.message),
        }

        Ok(())
    }
}

pub struct ListTokensCommand {}

impl Processable for ListTokensCommand {
    fn new() -> Self {
        Self {}
    }

    fn proceed(&mut self, _: &ArgMatches, client: &Client) -> Result<()> {
        let token_service =
            TokenService::from_client(client).chain_err(|| "Cannot create a valid TokenService")?;

        let tokens: Vec<Token> = token_service.list(Vec::new());
        let mut table = table!(["USER", "ID"]);

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        for t in tokens {
            table.add_row(Row::new(vec![Cell::new(&t.user), Cell::new(&t.id)]));
        }

        table.printstd();
        Ok(())
    }
}
