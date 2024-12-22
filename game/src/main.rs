mod logic;
mod utils;
mod network;


use libp2p::{
    Network,
    Transport,
    Multiaddr,
    PeerId,
};
use libp2p_webrtc::{
    WebRtcConfig,
    WebRtcTransport,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a WebRTC transport
    let transport = WebRtcTransport::new(WebRtcConfig::default()).await?;

    // Create a network with the WebRTC transport
    let mut network = Network::new(transport);

    // Get the local peer ID
    let local_peer_id = network.local_peer_id();
    println!("Your Peer ID: {:?}", local_peer_id);

    // Wait for the other peer's ID 
    println!("Paste the other peer's ID:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // Parse the input string into a PeerId
    let other_peer_id = PeerId::from_bytes(hex::decode(input.trim())?)?; 

    // ... (Rest of your connection logic) ...

    Ok(())
}