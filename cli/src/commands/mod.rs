use super::Result;
use clap::ArgMatches;
use api::client::Client;

pub struct Command<'a, T: Processable> {
    matches: &'a ArgMatches<'a>,
    client: &'a Client<'a>,
    inner: T,
}

impl<'a, T: Processable> Command<'a, T> {
    pub fn from_matches(matches: &'a ArgMatches, client: &'a Client) -> Self {
        Self {
            matches,
            client,
            inner: T::new(),
        }
    }

    pub fn proceed(&mut self) -> Result<()> {
        self.inner.proceed(self.matches, self.client)
    }
}

pub trait Processable {
    fn new() -> Self;
    fn proceed<'a>(&mut self, matches: &ArgMatches, client: &Client<'a>) -> Result<()>;
}

mod auth;
mod project;
mod job;
mod token;
mod run;

pub use self::auth::AuthCommand;
pub use self::run::RunCommand;
pub use self::project::ListProjectsCommand;
pub use self::job::ListJobsCommand;
pub use self::token::ListTokensCommand;
pub use self::token::TokenCreationCommand;
