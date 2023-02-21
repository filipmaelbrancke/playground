use std::io::Error;

use api::run;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    run()?.await
}
