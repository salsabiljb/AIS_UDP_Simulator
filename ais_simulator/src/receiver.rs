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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;
    use std::net::UdpSocket;
    use tokio::time::sleep;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_receive_ais_messages() -> Result<(), Box<dyn Error>> {
        let bind_addr = "192.168.1.74:50000";
        let output_file_path = "test_output.txt";

        // Spawn the receiver in a separate task
        tokio::spawn(async move {
            receive_ais_messages(bind_addr, output_file_path).await.unwrap();
        });

        // Give the receiver a moment to start
        sleep(Duration::from_millis(100)).await;

        // Send a test message
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        let message = "test message";
        socket.send_to(message.as_bytes(), bind_addr)?;

        // Give the receiver time to process the message
        sleep(Duration::from_millis(100)).await;

        // Verify the output
        let contents = fs::read_to_string(output_file_path).await?;
        assert!(contents.contains(message));

        // Clean up
        fs::remove_file(output_file_path).await?;

        Ok(())
    }
}
