use actix_juniper_cassandra::run;

fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = "8080";

    run(host, port)
}

