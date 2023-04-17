fn main() {
    if let Err(e) = rgrep::get_args().and_then(rgrep::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
