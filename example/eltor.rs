use libtor::{
    circuits, generate_hashed_password, get_circuits, HiddenServiceVersion, Tor, TorAddress,
    TorFlag,
};
use rand::Rng;
use std::error::Error;
use std::process::Command;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Generating hashed control password...");

    // Generate a 32-byte preimage
    let mut rng = rand::thread_rng();
    let preimage: [u8; 32] = rng.gen();
    let password: String = preimage
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    let hashed_password = generate_hashed_password(&password);
    let control_port = TorFlag::ControlPort(9051);
    let control_port_password = TorFlag::HashedControlPassword(hashed_password.trim().into());

    println!("Starting Tor...");
    let mut tor = Tor::new();

    tor.flag(TorFlag::DataDirectory("/tmp/tor-rust".into()))
        .flag(TorFlag::SocksPort(19050))
        .flag(control_port)
        .flag(control_port_password)
        .flag(TorFlag::HiddenServiceDir("/tmp/tor-rust/hs-dir".into()))
        .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
        .flag(TorFlag::HiddenServicePort(
            TorAddress::Port(8000),
            None.into(),
        ))
        .start_background();

    let flags = tor.get_flags();
    for flag in flags {
        println!("{:?}", flag);
    }

    sleep(Duration::from_secs(5)).await;
    
    // Call get_circuits
    match tor.get_circuits(&password).await {
        Ok(circuits) => {
            for circuit in circuits {
                println!("{:?}", circuit);
            }
        }
        Err(e) => {
            eprintln!("Failed to get circuits: {}", e);
        }
    }
    Ok(())
}
