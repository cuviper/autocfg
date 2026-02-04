fn main() {
    let mut args = std::env::args();
    // Add our own version so we can check that the wrapper is used for that.
    if args.any(|f| f == "--version") {
        println!("release: 12345.6789.0");
    }
}
