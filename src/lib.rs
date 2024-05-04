use std::error::Error;
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};
use flate2::{write::GzEncoder, read::GzDecoder};

const COMPRESSED_EXTENSIONS: [&str; 5] = ["gz", "zip", "tar", "rar", "7z"];

pub struct Config {
    pub filepath: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip the program name

        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get filepath"),
        };

        Ok(Config { filepath })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if is_compressed_file(&config.filepath) {
        let output_file = &config.filepath[..config.filepath.len() - 3];
        decompress_file(&config.filepath, output_file)?;
    } else {
        let output_file = format!("{}.gz", &config.filepath);
        compress_file(&config.filepath, &output_file)?;
    }

    Ok(())
}

fn is_compressed_file(filepath: &str) -> bool {
    let extension = filepath.split('.').last().unwrap();
    COMPRESSED_EXTENSIONS.contains(&extension)
}

fn compress_file(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let input = File::open(input_file)?;
    let mut input = BufReader::new(input);

    let output = File::create(output_file)?;
    let mut output = BufWriter::new(GzEncoder::new(output, flate2::Compression::default()));

    copy(&mut input,  &mut output)?;

    Ok(())
}

fn decompress_file(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let input = File::open(input_file)?;
    let mut input = BufReader::new(input);

    let output = File::create(output_file)?;
    let mut output = BufWriter::new(GzDecoder::new(output));

    copy(&mut input,  &mut output)?;

    Ok(())
}