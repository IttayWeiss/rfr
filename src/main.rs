fn main() {
    if let Err(e) = rfr::get_args().and_then(rfr::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
