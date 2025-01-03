use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Relay {
    fingerprint: String,
    name: String,
}

#[derive(Debug)]
pub struct Circuit {
    id: u32,
    status: String,
    relays: Vec<Relay>,
    build_flags: String,
    purpose: String,
    time_created: String,
}

pub async fn get_circuits(addr: &str, password: &str) -> Result<Vec<Circuit>, Box<dyn Error>> {
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

        // Parse the response into Circuit objects
        let circuits = parse_circuits(&circuits_info)?;
        return Ok(circuits);
    }

    println!("Failed to get response.");
    Err("Failed to get response".into())
}

fn parse_circuits(circuits_info: &str) -> Result<Vec<Circuit>, Box<dyn Error>> {
    let mut circuits = Vec::new();

    for line in circuits_info.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        }

        let id = parts[0].parse::<u32>()?;
        let status = parts[1].to_string();
        let path = parts[2..parts.len() - 3].join(" ");
        let build_flags = parts[parts.len() - 3].replace("BUILD_FLAGS=", "");
        let purpose = parts[parts.len() - 2].replace("PURPOSE=", "");
        let time_created = parts[parts.len() - 1].replace("TIME_CREATED=", "");

        let relays = parse_path(&path);

        circuits.push(Circuit {
            id,
            status,
            relays,
            build_flags,
            purpose,
            time_created,
        });
    }

    Ok(circuits)
}

fn parse_path(path: &str) -> Vec<Relay> {
    path.split(',')
        .filter_map(|pair| {
            let parts: Vec<&str> = pair.split('~').collect();
            if parts.len() == 2 {
                let fingerprint = parts[0].trim_start_matches('$').to_string();
                Some(Relay {
                    fingerprint,
                    name: parts[1].to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}