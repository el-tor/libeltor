use libtor::{HiddenServiceVersion, Tor, TorAddress, TorFlag, get_circuits, generate_hashed_password};
use std::error::Error;
use std::process::Command;
use tokio::time::{sleep, Duration};
use rand::Rng;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Generating hashed control password...");

    // Generate a 32-byte preimage
    let mut rng = rand::thread_rng();
    let preimage: [u8; 32] = rng.gen();
    let password: String = preimage.iter().map(|byte| format!("{:02x}", byte)).collect();

    let hashed_password = generate_hashed_password(&password);
    let control_port = TorFlag::ControlPort(9051);
    let control_port_password = TorFlag::HashedControlPassword(
        hashed_password.trim().into(),
    );

    println!("Starting Tor...");
    let tor = Tor::new()
        .flag(TorFlag::DataDirectory("/tmp/tor-rust".into()))
        .flag(TorFlag::SocksPort(19050))
        .flag(control_port)
        .flag(control_port_password)
        .flag(TorFlag::HiddenServiceDir("/tmp/tor-rust/hs-dir".into()))
        .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
        .flag(TorFlag::HiddenServicePort(
            TorAddress::Port(8000),
            None.into(),
        )).start_background();
        
    sleep(Duration::from_secs(2)).await;
    get_circuits("127.0.0.1:9051", &password).await?; 

    Ok(())
}
