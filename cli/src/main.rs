//! command-line on Polymath.

use clap::{
    command,
    error::{ContextKind, ContextValue, ErrorKind},
    Error, Parser as _,
};
use clap_derive::{Parser, Subcommand};
use url::Url;

/// Max depth value allowed on cli.
const MAX_DEPTH: usize = 100;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Start crawling a website.
    Crawl {
        url: Url,
        /// Number of crawled pages.
        #[arg(short, long)]
        max_depth: Option<usize>,
        /// Whether we follow `/robots.txt`.
        #[arg(short, long)]
        robots_txt: Option<bool>,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Crawl {
            url,
            max_depth,
            robots_txt,
        } => {
            let max_depth = max_depth.unwrap_or(1);

            if max_depth > MAX_DEPTH {
                let mut err = Error::new(ErrorKind::ValueValidation);
                err.insert(
                    ContextKind::InvalidArg,
                    ContextValue::String("--max-depth".to_owned()),
                );
                err.insert(
                    ContextKind::InvalidValue,
                    ContextValue::Number(
                        max_depth.try_into().unwrap_or_default(),
                    ),
                );
                err.insert(
                    ContextKind::Usage,
                    ContextValue::String(format!(
                        "Value cannot exceed {}",
                        MAX_DEPTH
                    )),
                );
                err.exit();
            }

            println!(
                "{} // {:?} // {:?}",
                url,
                max_depth,
                robots_txt.unwrap_or(true)
            )
        },
    }
}
