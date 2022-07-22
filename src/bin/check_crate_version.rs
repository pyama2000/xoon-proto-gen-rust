fn main() -> std::process::ExitCode {
    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Wrong usage");
        return 1.into()
    }
    let latest_tag = args.get(1).unwrap();
    dbg!(&version);
    dbg!(&latest_tag);
    if version.eq(latest_tag) {
        eprintln!("Update crate version!");
        return 1.into()
    }

    0.into()
}
