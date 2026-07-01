use crate::Sample;
use cortex::Format;
use std::path::PathBuf;
use std::time::SystemTime;

pub struct ScanReport {
    pub path: PathBuf,
    pub format: Format,
    pub size_bytes: u64,
    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
    // pub sample: SampleReport,
}
