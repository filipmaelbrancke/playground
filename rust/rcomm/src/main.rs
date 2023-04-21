fn main() {
    if let Err(e) = rcomm::get_args().and_then(rcomm::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
