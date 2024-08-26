use eyre::{Context, ContextCompat, Result};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;
use std::path::PathBuf;

use crate::cmd::{get_commit_hash, get_cuda_version, get_x86_features};

pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn get_current_dir() -> Result<PathBuf> {
    let current_dir = env::current_exe().context("current_dir")?;
    let current_dir = current_dir.parent().context("current dir parent")?;
    Ok(current_dir.to_path_buf())
}

pub fn get_issue_url(logs: String) -> String {
    println!("{}", logs);
    format!("https://github.com/{}/samwise/issues/new", "chandeldivyam")
}

pub trait LogError<T> {
    fn log_error(self) -> Option<T>;
}

impl<T> LogError<T> for Result<T> {
    fn log_error(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(ref e) => {
                tracing::error!("Error: {:?}", e);
                None
            }
        }
    }
}
