fn main() {
    if let Err(e) = rls::get_args().and_then(rls::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
