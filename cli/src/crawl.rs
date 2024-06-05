//! handle crawler command.

use clap::{
    error::{ContextKind, ContextValue, ErrorKind},
    Error,
};
use std::{fs, path::PathBuf};

/// Max depth value allowed on cli.
const MAX_DEPTH: usize = 100;

pub fn handler(
    url: url::Url,
    depth: usize,
    _robots_txt: bool,
    path: Option<PathBuf>,
) -> Result<(), clap::error::Error> {
    if depth > MAX_DEPTH {
        let mut err = Error::new(ErrorKind::ValueValidation);
        err.insert(
            ContextKind::InvalidArg,
            ContextValue::String("--max-depth".to_owned()),
        );
        err.insert(
            ContextKind::InvalidValue,
            ContextValue::Number(depth.try_into().unwrap_or_default()),
        );
        err.insert(
            ContextKind::Usage,
            ContextValue::String(format!("Value cannot exceed {}", MAX_DEPTH)),
        );
        err.exit();
    }

    // Announce what crawler will do.
    println!(
        "Crawling {} with a maximum of {} crawled pages.",
        url, depth
    );
    
    if let Some(path) = path {
        println!(
            "Saving pages result on {}",
            path.to_str().unwrap_or_default()
        );

        fs::create_dir(path)?;
    }

    Ok(())
}
