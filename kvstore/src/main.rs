fn main() {
    if let Err(e) = kvstore::get_args().and_then(kvstore::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
