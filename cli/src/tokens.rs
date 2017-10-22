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
        table.add_row(Row::new(vec![Cell::new(&t.user), Cell::new(&t.id)]));
    }

    table.printstd();
}

pub fn new(service: &TokenService, user: &str, duration: Option<&str>, roles: Vec<&str>) {
    let body = TokenBody {
        user: user.into(),
        roles: roles.into_iter().map(|x| x.into()).collect::<_>(),
        duration: duration.unwrap_or("").into(),
    };

    let token = service.create(&body);

    match token {
        Ok(t) => println!("{}", t.id),
        Err(e) => println!("Error: {}", e.message),
    }
}
