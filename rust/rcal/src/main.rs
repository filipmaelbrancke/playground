fn main() {
    if let Err(e) = rcal::get_args().and_then(rcal::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
