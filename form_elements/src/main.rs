#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, FormBody};
use std::collections::HashMap;

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! { |_, res|
        let mut data = HashMap::new();
        data.insert("title", "Form");

        return res.render("elements/form.html", &data);
    });

    server.post("/result", middleware! { |req, res|
        let form_data = try_with!(res, req.form_body());

        println!("{:?}", form_data);

        let mut data = HashMap::new();
        data.insert("title", "Result");
        data.insert("hidden", form_data.get("hidden").unwrap_or("Hidden?"));
        data.insert("text", form_data.get("text").unwrap_or("Text?"));
        data.insert("password", form_data.get("password").unwrap_or("Password?"));
        data.insert("email", form_data.get("email").unwrap_or("Email?"));
        data.insert("checkbox", form_data.get("checkbox").unwrap_or("Checkbox?"));
        data.insert("radio", form_data.get("radio").unwrap_or("Radio?"));
        data.insert("color", form_data.get("color").unwrap_or("Color?"));
        data.insert("date", form_data.get("date").unwrap_or("Date?"));
        data.insert("number", form_data.get("number").unwrap_or("Number?"));
        data.insert("select", form_data.get("select").unwrap_or("Select?"));
        data.insert("datalist", form_data.get("datalist").unwrap_or("Datalist?"));
        data.insert("textarea", form_data.get("textarea").unwrap_or("Textarea?"));
        data.insert("progress", form_data.get("progress").unwrap_or("Progress?"));
        data.insert("meter", form_data.get("meter").unwrap_or("meter"));
        data.insert("file", form_data.get("file").unwrap_or("file"));

        return res.render("elements/result.html", &data);
    });

    server.listen("127.0.0.1:6565").unwrap();
}
