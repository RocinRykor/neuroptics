use std::env;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("usage: playground <subcommand> [args...]");
        return;
    }

    let subcommand = args.remove(0);
    // let flags = extract_flags(&mut args);
}