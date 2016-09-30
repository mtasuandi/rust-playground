#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, FormBody};
use std::collections::HashMap;

fn main() {
    let mut server = Nickel::new();

    server.get("/login", middleware! { |_, res|
        let mut data = HashMap::new();
        data.insert("title", "Login");

        return res.render("views/login.html", &data);
    });

    server.post("/confirmation", middleware! { |req, res|
        let form_data = try_with!(res, req.form_body());

        println!("{:?}", form_data);

        let mut data = HashMap::new();
        data.insert("title", "Confirmation");
        data.insert("username", form_data.get("username").unwrap_or("Username?"));
        data.insert("password", form_data.get("password").unwrap_or("Password?"));

        return res.render("views/confirmation.html", &data);
    });

    server.listen("127.0.0.1:6565").unwrap();
}
