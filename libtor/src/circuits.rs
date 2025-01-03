use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub async fn get_circuits(addr: &str, password: &str) -> Result<String, Box<dyn Error>> {
    println!("Connecting to Tor control port...");
    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = tokio::io::split(stream);
    let mut reader = BufReader::new(reader);

    // Authenticate with the control port using the provided control cookie
    println!("Authenticating...");
    let auth_command = format!("AUTHENTICATE \"{}\"\r\n", password);
    writer.write_all(auth_command.as_bytes()).await?;
    writer.flush().await?;

    let mut response = String::new();
    reader.read_line(&mut response).await?;
    if !response.starts_with("250") {
        return Err("Authentication failed".into());
    }

    // Send the GETINFO circuits command
    println!("Sending GETINFO circuits command...");
    writer.write_all(b"GETINFO circuit-status\r\n").await?;
    writer.flush().await?;

    // Read the response
    println!("Reading response...");
    response.clear();
    reader.read_line(&mut response).await?;
    if response.starts_with("250") {
        let mut circuits_info = String::new();
        while reader.read_line(&mut circuits_info).await? > 0 {
            if circuits_info.ends_with(".\r\n") {
                break;
            }
        }
        return Ok(circuits_info);
    }

    println!("Failed to get response.");
    Err("Failed to get response".into())
}
