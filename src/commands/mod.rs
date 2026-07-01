pub mod scan;

pub use scan::Scan;
use std::process::Command;

struct Cli {
    command: Command,
}
