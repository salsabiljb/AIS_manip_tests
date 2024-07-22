use ais::{AisFragments, AisParser};
use ais::messages::AisMessage;
use std::io::{self, BufRead};

fn main() -> Result<(), ais::errors::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut parser = AisParser::new();

    for line in handle.lines() {
        let line = line.map_err(|e| ais::errors::Error::from(e.to_string()))?;
        let line_bytes = line.as_bytes();

        if let AisFragments::Complete(sentence) = parser.parse(line_bytes, true)? {
            // Print the raw AIS message
            println!("Raw AIS message: {}", line);

            if let Some(message) = sentence.message {
                match message {
                    AisMessage::PositionReport(report) => {
                        println!("Parsed AIS Position Report: {:?}", report);
                    },
                    AisMessage::BaseStationReport(report) => {
                        println!("Parsed AIS Base Station Report: {:?}", report);
                    },
                    AisMessage::BinaryBroadcastMessage(report) => {
                        println!("Parsed AIS Binary Broadcast Message: {:?}", report);
                    },
                    AisMessage::Interrogation(report) => {
                        println!("Parsed AIS Interrogation: {:?}", report);
                    },
                    AisMessage::StaticAndVoyageRelatedData(report) => {
                        println!("Parsed AIS Static and Voyage Related Data: {:?}", report);
                    },
                    AisMessage::DgnssBroadcastBinaryMessage(report) => {
                        println!("Parsed AIS DGNSS Broadcast Binary Message: {:?}", report);
                    },
                    AisMessage::StandardClassBPositionReport(report) => {
                        println!("Parsed AIS Standard Class B Position Report: {:?}", report);
                    },
                    AisMessage::ExtendedClassBPositionReport(report) => {
                        println!("Parsed AIS Extended Class B Position Report: {:?}", report);
                    },
                    AisMessage::DataLinkManagementMessage(report) => {
                        println!("Parsed AIS Data Link Management Message: {:?}", report);
                    },
                    AisMessage::AidToNavigationReport(report) => {
                        println!("Parsed AIS Aid to Navigation Report: {:?}", report);
                    },
                    AisMessage::StaticDataReport(report) => {
                        println!("Parsed AIS Static Data Report: {:?}", report);
                    },
                    AisMessage::UtcDateResponse(report) => {
                        println!("Parsed AIS UTC Date Response: {:?}", report);
                    },
                }
            } else {
                println!("Received an incomplete or unsupported message");
            }
        } else {
            println!("Received an incomplete or malformed AIS message");
        }
    }

    Ok(())
}

//nc 153.44.253.27 5631 | cargo run
