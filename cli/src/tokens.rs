use prettytable::format;
use prettytable::row::Row;
use prettytable::cell::Cell;
use api::Token;
use api::token::TokenBody;
use api::TokenService;

pub fn list_tokens(service: &TokenService) {

    let tokens: Vec<Token> = service.list(Vec::new());
    let mut table = table!(["USER", "ID"]);

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    for t in tokens {
        table.add_row(Row::new(vec![
                               Cell::new(&t.user),
                               Cell::new(&t.id)]));
    }

    table.printstd();
}

pub fn new(service: &TokenService, user: &str, duration: Option<&str>, roles:Vec<&str>) {

    let body = TokenBody {
        user: user.to_string(),
        roles: roles.iter().cloned().map(|x| x.to_string()).collect::<Vec<String>>(),
        duration: duration.unwrap_or("").to_string()
    };

    let token = service.new(&body);

    match token {
        Ok(t) => println!("{}", t.id),
        Err(e) => println!("Error: {}", e.message)
    }
}
