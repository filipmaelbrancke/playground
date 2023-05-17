fn main() {
    if let Err(e) = rfortune::get_args().and_then(rfortune::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
