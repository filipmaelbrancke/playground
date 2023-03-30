fn main() {
    if let Err(e) = rfind::get_args().and_then(rfind::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
