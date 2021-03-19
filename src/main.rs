use actix_juniper_cassandra::run;

use std::env::{var};

fn main() -> std::io::Result<()> {
    let host: String = var("HOST").unwrap_or("127.0.0.1".to_string());
    let port: String = var("PORT").unwrap_or("8080".to_string());

    // dynamically allocate 'static variables
    run(Box::leak(host.into_boxed_str()), Box::leak(port.into_boxed_str()))
}

