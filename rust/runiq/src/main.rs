fn main() {
    if let Err(e) = runiq::get_args().and_then(runiq::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
