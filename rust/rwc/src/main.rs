fn main() {
    if let Err(e) = rwc::get_args().and_then(rwc::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
