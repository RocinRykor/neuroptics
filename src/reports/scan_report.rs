use std::path::PathBuf;
use std::time::SystemTime;
use cortex::Format;
use crate::Sample;

pub struct ScanReport {
    pub path: PathBuf,
    pub format: Format,
    pub size_bytes: u64,
    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
    // pub sample: SampleReport,
}