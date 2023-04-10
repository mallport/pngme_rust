use std::path::PathBuf;

use crate::chunk_type::ChunkType;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub mode: PngMeArgs,
}

#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args)]
#[clap(author, version, about, long_about = None)]
pub struct EncodeArgs {
    #[clap(value_parser)]
    pub path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
    #[clap(value_parser)]
    pub message: String,
}

#[derive(Args)]
#[clap(author, version, about, long_about = None)]
pub struct DecodeArgs {
    #[clap(value_parser)]
    pub path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args)]
#[clap(author, version, about, long_about = None)]
pub struct RemoveArgs {
    #[clap(value_parser)]
    pub path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args)]
#[clap(author, version, about, long_about = None)]
pub struct PrintArgs {
    #[clap(value_parser)]
    pub path: PathBuf,
}
