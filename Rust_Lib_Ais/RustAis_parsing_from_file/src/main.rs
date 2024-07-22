use ais::{AisFragments, AisParser};
use ais::messages::AisMessage;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::time::Instant;

fn main() -> Result<(), ais::errors::Error> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Err(ais::errors::Error::from("Invalid number of arguments"));
    }
    let file_path = &args[1];

    // Open the file
    let file = File::open(file_path)
        .map_err(|e| ais::errors::Error::from(format!("Failed to open file: {}", e)))?;
    let reader = BufReader::new(file);

    let mut parser = AisParser::new();
    let mut parsed_count = 0;
    let mut unparsed_count = 0;

    // Start timing
    let start_time = Instant::now();

    // Read and parse each line in the file
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                unparsed_count += 1;
                continue;
            }
        };
        let line_bytes = line.as_bytes();

        match parser.parse(line_bytes, true) {
            Ok(AisFragments::Complete(sentence)) => {
                if let Some(message) = sentence.message {
                    match message {
                        AisMessage::PositionReport(_)
                        | AisMessage::BaseStationReport(_)
                        | AisMessage::BinaryBroadcastMessage(_)
                        | AisMessage::Interrogation(_)
                        | AisMessage::StaticAndVoyageRelatedData(_)
                        | AisMessage::DgnssBroadcastBinaryMessage(_)
                        | AisMessage::StandardClassBPositionReport(_)
                        | AisMessage::ExtendedClassBPositionReport(_)
                        | AisMessage::DataLinkManagementMessage(_)
                        | AisMessage::AidToNavigationReport(_)
                        | AisMessage::StaticDataReport(_)
                        | AisMessage::UtcDateResponse(_) => {
                            parsed_count += 1;
                        },
                    }
                } else {
                    unparsed_count += 1;
                }
            }
            Ok(_) => {
                unparsed_count += 1;
            }
            Err(e) => {
                eprintln!("Failed to parse message: {}", e);
                unparsed_count += 1;
            }
        }
    }

    // Calculate elapsed time
    let elapsed_time = start_time.elapsed().as_secs_f64();

    // Print statistics
    println!("Messages provided: {}", parsed_count + unparsed_count);
    println!("Parsed messages: {}", parsed_count);
    println!("Unparsed messages: {}", unparsed_count);
    println!("Time taken to parse messages: {:.2} seconds", elapsed_time);

    Ok(())
}
