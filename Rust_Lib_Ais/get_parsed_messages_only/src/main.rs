use ais::{AisFragments, AisParser};
use ais::messages::AisMessage;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::env;

fn main() -> Result<(), ais::errors::Error> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file_path> <output_file_path>", args[0]);
        return Err(ais::errors::Error::from("Invalid number of arguments"));
    }
    let input_file_path = &args[1];
    let output_file_path = &args[2];

    // Open the input file
    let input_file = File::open(input_file_path)
        .map_err(|e| ais::errors::Error::from(format!("Failed to open input file: {}", e)))?;
    let reader = BufReader::new(input_file);

    // Open the output file
    let mut output_file = File::create(output_file_path)
        .map_err(|e| ais::errors::Error::from(format!("Failed to create output file: {}", e)))?;

    let mut parser = AisParser::new();

    // Read and parse each line in the input file
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
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
                            // Only write the line if the message is fully parsed
                            if let Err(e) = writeln!(output_file, "{}", line) {
                                eprintln!("Failed to write to output file: {}", e);
                            }
                        },
                    }
                }
            }
            Ok(_) => {
                // Do not save incomplete or unsupported messages
                eprintln!("Received an incomplete or unsupported message");
            }
            Err(e) => {
                // Do not save messages that caused a parsing error
                eprintln!("Failed to parse message: {}", e);
            }
        }
    }

    Ok(())
}
