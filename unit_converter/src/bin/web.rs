use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

use unitverse;

#[derive(Debug, Deserialize, PartialEq, Copy, Clone)]
enum TemperatureUnit {
    C,
    F,
}

#[derive(Deserialize)]
struct ConversionInput {
    n: f64,
    f: TemperatureUnit, // from
    t: TemperatureUnit, // to
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_homepage))
            .route("/convert", web::post().to(post_convert))
    });

    println!("Serviing on http://localhost:3000...");

    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .expect("error running server");
}

fn get_homepage() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>I Convert</title>
        <form action="/convert" method="post">
        <label>From</label>
        <input type="text" name="n"/>
        <select name="f">
            <option value="F">F</option>
            <option value="C">C</option>
        </select>
        <label>to</label>
        <select name="t">
            <option value="C">C</option>
            <option value="F">F</option>
        </select>
        <button type="submit">Convert</button>
        "#,
    )
}

fn convert_temperature(conversion_input: ConversionInput) -> f64 {
    match conversion_input {
        ConversionInput {
            n,
            f: TemperatureUnit::F,
            t: TemperatureUnit::C,
        } => unitverse::farenheit_to_celsius(n),
        ConversionInput {
            n,
            f: TemperatureUnit::C,
            t: TemperatureUnit::F,
        } => unitverse::celsius_to_farenheit(n),
        ConversionInput {
            n,
            f: TemperatureUnit::F,
            t: TemperatureUnit::F,
        } => n,
        ConversionInput {
            n,
            f: TemperatureUnit::C,
            t: TemperatureUnit::C,
        } => n,
    }
}

fn post_convert(form: web::Form<ConversionInput>) -> HttpResponse {
    let response = format!(
        "Result: {} {:?} => {} {:?}\n",
        form.n,
        form.f,
        convert_temperature(ConversionInput {
            n: form.n,
            f: form.f,
            t: form.t
        }),
        form.t
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}
