use std::fs::File;
use std::io::{Read, Write};

use clap::Parser;

#[derive(Parser)]
#[command(name = "Volume", about = "WAV File Volume Scaler")]
struct Args {
    input: String,
    output: String,
    factor: f32,
}

const HEADER_SIZE: usize = 44;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Open input.wav for reading (binary mode)
    let mut input_file = File::open(format!("volume/{}", &args.input))?;

    // Create fixed-sized array of HEADER_SIZE with bytes set to 0
    // This will store the 44-byte WAV file header
    let mut header = [0u8; HEADER_SIZE];

    // Read exactly 44 bytes from the beginning of the file into `header`
    input_file.read_exact(&mut header)?;

    // Create a vector to hold the rest of the file (the sample data)
    let mut samples = Vec::new();

    // Read the remaining bytes (audio samples) into the `samples` vector
    input_file.read_to_end(&mut samples)?;

    // Open/create output.wav for writing
    let mut output_file = File::create(format!("volume/{}", &args.output))?;

    // Write the original header to the output file unchanged
    output_file.write_all(&header)?;

    // Iterate through the sample data in 2-byte chunks (16-bit samples)
    for chunk in samples.chunks_exact(2) {
        // Convert the 2-byte chunk to a little-endian i16 audio sample
        let sample = i16::from_le_bytes([chunk[0], chunk[1]]);

        // Scale the sample by the given factor
        // Clamp it to the valid i16 range to prevent overflow/distortion
        let scaled = (sample as f32 * args.factor).clamp(i16::MIN as f32, i16::MAX as f32) as i16;

        // Write the scaled sample back as little-endian bytes
        output_file.write_all(&scaled.to_le_bytes())?;
    }

    Ok(())
}
