use std::io::Error;
use std::net::TcpListener;

use api::run;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
