pub mod scan;

use std::process::Command;
pub use scan::Scan;

struct Cli {
    command: Command,
}