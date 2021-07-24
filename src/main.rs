/// Simple web server: it'll serve only a page that prompts
/// a user for numbers to compute with.
use std::env; // Brings the env module into the scope
use std::str::FromStr; // Brings the trait FromStr
                       // The trait is a collection of methods that types can implement
                       // The trait must(!) be in scope in order to use its methods
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .expect("error running server");

    let mut numbers = Vec::new(); // Creating a vector, it has to be mutable
                                  // in order to push values onto the end of it

    for arg in env::args().skip(1) {
        // env::args() returns an ITERATOR
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
        // from_str method - it's a assotiated method (similar to
        // static methods in C++ and Java)
        // this method returns Result value (which enum with Err() and Ok() variants)
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

// Computes the greatest common divisor of two integers, using Euclid's algorithm
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// Writing unit tests
#[test] // It is an attribute:
        // open-ended system for marking
        // func and other declarations with extra information
fn gcd_returns_correct_value() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
        "#,
    )
}

// Structure defined below represents the values expected
// from a web-form
#[derive(Deserialize)] // the attribute tells the `serde` crate
                       // to exemine below type when the program is compiled
                       // and automatically generate code to parse a value
                       // of this type from data in the format that HTML forms
                       // use for POST requests.
struct GcdParams {
    n: u64,
    m: u64,
}

fn post_gcd(form: web::Form<GcdParams>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    // the `format!` macro is just like `println!`, except that instead of writing
    // the text to the stdout, it returns is is a string.
    let response = format!(
        "The greatest common divisor of the numbers {} and {} \
                            is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}
