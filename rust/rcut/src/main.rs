fn main() {
    if let Err(e) = rcut::get_args().and_then(rcut::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
