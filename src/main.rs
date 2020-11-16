use anyhow::{anyhow, Result};
use etherparse::PacketHeaders;
use win_rawcap::create_raw_socket;

#[tokio::main]
async fn main() -> Result<()> {
    // Create socket
    let socket = create_raw_socket()?;

    let mut buf = [0u8; (256 * 256) - 1];
    loop {
        let len = socket.recv(&mut buf).await?;
        let mut packet = PacketHeaders::from_ip_slice(&buf[0..len])?;
        println!("{:?}", packet.transport);
    }

    Ok(())
}
