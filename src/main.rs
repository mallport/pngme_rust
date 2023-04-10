#![allow(dead_code, unused_imports)]
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    match &cli.mode {
        args::PngMeArgs::Encode(e) => commands::encode(e),
        args::PngMeArgs::Decode(d) => commands::decode(d),
        args::PngMeArgs::Remove(r) => commands::remove(r),
        args::PngMeArgs::Print(p) => commands::print_chunks(p),
    }?;
    Ok(())
}
