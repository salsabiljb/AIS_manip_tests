use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::fs::OpenOptions;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "153.44.253.27:5631";
    let stream = TcpStream::connect(addr).await?;
    println!("Connected to {}", addr);

    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    // Open the file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("ais_received.txt")
        .await?;

    while let Some(line) = lines.next_line().await? {
        let message = line.trim();
        println!("Received: {}", message);

        file.write_all(format!("{}\n", message).as_bytes()).await?;
    }

    Ok(())
}


