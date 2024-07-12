use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::net::UdpSocket;
use std::error::Error;

pub async fn receive_ais_messages(bind_addr: &str, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind(bind_addr).await?;
    println!("Listening on {}", bind_addr);

    let mut buf = vec![0; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[..len]);
        println!("Received from {}: {}", addr, message);

        // Save the received message to the file
        let mut file = OpenOptions::new().create(true).append(true).open(output_file_path).await?;
        file.write_all(message.as_bytes()).await?;
        file.write_all(b"\n").await?;
    }
}
