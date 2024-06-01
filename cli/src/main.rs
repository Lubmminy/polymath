//! command-line on Polymath.

mod crawl;

use clap::{command, Parser as _};
use clap_derive::{Parser, Subcommand};
use std::path::PathBuf;
use url::Url;

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
        /// Whether crawler follow `/robots.txt`.
        #[arg(long)]
        robots_txt: Option<bool>,
        /// Directory for saving HTML content of pages.
        /// 
        /// If not set, do not save anything.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Crawl {
            url,
            max_depth,
            robots_txt,
            path,
        } => {
            if let Err(error) = crawl::handler(
                url,
                max_depth.unwrap_or(1),
                robots_txt.unwrap_or(true),
                path,
            ) {
                error.exit();
            }
        },
    }
}
