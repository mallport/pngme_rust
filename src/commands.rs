use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};
use anyhow::Error;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: &EncodeArgs) -> Result<(), Error> {
    let mut png = Png::from_file(&args.path)?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.clone().into_bytes());
    png.append_chunk(chunk);

    let output_path = &mut args.path.clone();
    let new_path = "encoded-".to_owned() + (&output_path.file_name().unwrap().to_str().unwrap());
    output_path.set_file_name(new_path);

    fs::write(output_path, png.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: &DecodeArgs) -> Result<(), Error> {
    let png = Png::from_file(&args.path)?;
    let chunk = png.chunk_by_type(&args.chunk_type).unwrap();

    println!("{}", chunk.data_as_string()?);
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: &RemoveArgs) -> Result<(), Error> {
    let mut png = Png::from_file(&args.path)?;
    png.remove_chunk(&args.chunk_type)?;

    fs::write(&args.path, png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: &PrintArgs) -> Result<(), Error> {
    let png = Png::from_file(&args.path)?;
    println!("{}", png);

    Ok(())
}
