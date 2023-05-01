fn main() {
    if let Err(e) = rtail::get_args().and_then(rtail::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
