use std::fmt::Display;
use super::Result;

pub trait Command {
    fn proceed(&mut self) -> Result<()>;
    fn is_quiet(&self) -> bool;

    fn println<S: Display>(&self, message: S) {
        if !self.is_quiet() {
            println!("{}", message);
        }
    }
    fn print<S: Display>(&self, message: S) {
        if !self.is_quiet() {
            print!("{}", message);
        }
    }
}

mod auth;

pub use self::auth::AuthCommand;
