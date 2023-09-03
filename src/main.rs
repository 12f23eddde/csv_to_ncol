use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write, Seek};
use chrono::NaiveDateTime;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the input and output file paths are provided
    if args.len() < 3 {
        println!("Usage: cargo run <input_file> <output_file>");
        return Ok(());
    }

    // Get the input and output file paths from the command-line arguments
    let input_file = &args[1];
    let output_file = &args[2];

    // Open the input file
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);

    // Create the output file
    let output_file = File::create(output_file)?;
    let mut writer = BufWriter::new(output_file);

    // Reset the reader to the beginning of the file
    reader.seek(std::io::SeekFrom::Start(0))?;

    // Process each line in the input file
    for (index, line) in reader.lines().enumerate().skip(1) {
        let line = line?;
        let fields: Vec<&str> = line.split(',').map(|s| s.trim_matches('"')).collect();

        // Extract the vertex names and timestamp
        let vertex1 = fields[0];
        let vertex2 = fields[1];
        let timestamp_str = fields[2];

        // Parse the timestamp string into a NaiveDateTime
        let timestamp = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S")?;

        // Convert the timestamp to a Unix timestamp
        let unix_timestamp = timestamp.timestamp();

        // check if vertex1, vertex2, and unix_timestamp are valid
        if vertex1.is_empty() || vertex2.is_empty() || unix_timestamp < 0 {
            println!("Invalid line: {}", line);
            continue;
        }

        // Write the converted line to the output file
        let converted_line = format!("{} {} {}\n", vertex1, vertex2, unix_timestamp);
        writer.write_all(converted_line.as_bytes())?;

        if index % 1000000 == 0 {
            println!("Processing line {}", index);
        }
    }

    // Flush the buffer to ensure all data is written to the file
    writer.flush()?;

    println!("Conversion completed successfully.");
    Ok(())
}
