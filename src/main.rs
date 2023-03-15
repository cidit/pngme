use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

use clap::{arg, value_parser, Command};
use pngme::{
    chunk::{Chunk, ChunkType},
    png::Png,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let matches = cli().get_matches();
    match Commands::from(matches) {
        Commands::Encode {
            path,
            chunk_type,
            message,
            out_file,
        } => {
            let mut input = Vec::new();
            File::open(&path)?.read_to_end(&mut input)?;
            let mut png = Png::try_from(input.as_slice())?;
            let new_chunk = Chunk::new(chunk_type, message.into_bytes().to_vec());
            png.append_chunk(new_chunk);
            let path = out_file.unwrap_or(path);
            let mut file = File::create(path)?;
            file.write_all(&png.as_bytes())?;
        }
        Commands::Decode { path, chunk_type } => {
            let mut input = Vec::new();
            File::open(&path)?.read_to_end(&mut input)?;
            let png = Png::try_from(input.as_slice())?;
            let chunk = png.chunk_by_type(&chunk_type.to_string());
            let out = chunk
                .map(|c| c.data())
                .map(|d| String::from_utf8_lossy(d))
                .unwrap_or("Not found".into());
            println!("{}", out);
        }
        Commands::Remove { path, chunk_type } => {
            let mut input = Vec::new();
            File::open(&path)?.read_to_end(&mut input)?;
            let mut png = Png::try_from(input.as_slice())?;
            png.remove_chunk(chunk_type.to_string().as_str())?;
            let mut file = File::create(path)?;
            file.write_all(&png.as_bytes())?;
        }
        Commands::Print { path } => {
            let mut input = Vec::new();
            File::open(&path)?.read_to_end(&mut input)?;
            let png = Png::try_from(input.as_slice())?;
            for chunk in png.chunks() {
                println!("{}", chunk)
            }
        }
    }
    println!("Job done");
    Ok(())
}

fn cli() -> Command {
    Command::new("pngme")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("encode")
                .arg(arg!(<PATH>).value_parser(value_parser!(PathBuf)))
                .arg(arg!(<CHUNK_TYPE>))
                .arg(arg!(<MESSAGE>))
                .arg(arg!([OUTFILE]).value_parser(value_parser!(PathBuf))),
            Command::new("decode")
                .arg(arg!(<PATH>).value_parser(value_parser!(PathBuf)))
                .arg(arg!(<CHUNK_TYPE>)),
            Command::new("remove")
                .arg(arg!(<PATH>).value_parser(value_parser!(PathBuf)))
                .arg(arg!(<CHUNK_TYPE>)),
            Command::new("print").arg(arg!(<PATH>).value_parser(value_parser!(PathBuf))),
        ])
}

enum Commands {
    Encode {
        path: PathBuf,
        chunk_type: ChunkType,
        message: String,
        out_file: Option<PathBuf>,
    },
    Decode {
        path: PathBuf,
        chunk_type: ChunkType,
    },
    Remove {
        path: PathBuf,
        chunk_type: ChunkType,
    },
    Print {
        path: PathBuf,
    },
}

impl From<clap::ArgMatches> for Commands {
    fn from(matches: clap::ArgMatches) -> Self {
        match matches.subcommand() {
            Some(("encode", sub_matches)) => {
                let path = sub_matches
                    .get_one::<PathBuf>("PATH")
                    .expect("required")
                    .clone();
                let chunk_type = sub_matches
                    .get_one::<String>("CHUNK_TYPE")
                    .expect("required");
                let message = sub_matches
                    .get_one::<String>("MESSAGE")
                    .expect("required")
                    .clone();
                let out_file = sub_matches.get_one::<PathBuf>("OUTFILE").cloned();
                Self::Encode {
                    path,
                    chunk_type: ChunkType::from_str(chunk_type).unwrap(),
                    message,
                    out_file,
                }
            }
            Some(("decode", sub_matches)) => {
                let path = sub_matches
                    .get_one::<PathBuf>("PATH")
                    .expect("required")
                    .clone();
                let chunk_type = sub_matches
                    .get_one::<String>("CHUNK_TYPE")
                    .expect("required");
                Self::Decode {
                    path,
                    chunk_type: ChunkType::from_str(chunk_type).unwrap(),
                }
            }
            Some(("remove", sub_matches)) => {
                let path = sub_matches
                    .get_one::<PathBuf>("PATH")
                    .expect("required")
                    .clone();
                let chunk_type = sub_matches
                    .get_one::<String>("CHUNK_TYPE")
                    .expect("required");
                Self::Remove {
                    path,
                    chunk_type: ChunkType::from_str(chunk_type).unwrap(),
                }
            }
            Some(("print", sub_matches)) => {
                let path = sub_matches
                    .get_one::<PathBuf>("PATH")
                    .expect("required")
                    .clone();
                Self::Print { path }
            }
            _ => unreachable!(),
        }
    }
}
