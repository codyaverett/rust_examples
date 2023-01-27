#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, Nickel};

fn main() {
    let mut server = Nickel::new();

    server.get("/bar", middleware!("This is the /bar handler"));

    server.get(
        "/user/:userid",
        middleware! { |request|
            format!("This is user: {:?}", request.param("userid"))
        },
    );

    server.get("/a/*/d", middleware!("matches /a/b/d but not /a/b/c/d"));

    server.get(
        "/a/**/d",
        middleware!("This matches /a/b/d and also /a/b/c/d"),
    );

    server.listen("127.0.0.1:6767").unwrap();
}
