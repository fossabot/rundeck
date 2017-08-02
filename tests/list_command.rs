extern crate rundeck;
extern crate reqwest;

#[macro_use]
mod server;

use std::io::Read;

#[test]
fn test_get() {
    let server = server! {
        request: b"\
            GET /1 HTTP/1.1\r\n\
            Host: $HOST\r\n\
            User-Agent: $USERAGENT\r\n\
            Accept: */*\r\n\
            Accept-Encoding: gzip\r\n\
            \r\n\
            ",
        response: b"\
            HTTP/1.1 200 OK\r\n\
            Server: test\r\n\
            Content-Length: 0\r\n\
            \r\n\
            "
    };

    let url = format!("http://{}/1", server.addr());
    let mut res = reqwest::get(&url).unwrap();

    assert_eq!(res.url().as_str(), &url);
    assert_eq!(res.status(), reqwest::StatusCode::Ok);
    assert_eq!(res.headers().get(),
               Some(&reqwest::header::Server::new("test".to_string())));
    assert_eq!(res.headers().get(),
               Some(&reqwest::header::ContentLength(0)));

    let mut buf = [0; 1024];
    let n = res.read(&mut buf).unwrap();
    assert_eq!(n, 0)
}
