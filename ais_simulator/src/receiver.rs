use std::error::Error;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::net::UdpSocket;

pub async fn receive_ais_messages(
    bind_addr: &str,
    output_file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind(bind_addr).await?;
    println!("Listening on {}", bind_addr);

    let mut buf = vec![0; 1024];

    loop {
        let (len, addr) = match socket.recv_from(&mut buf).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Failed to receive message: {}", e);
                continue;
            }
        };

        let message = String::from_utf8_lossy(&buf[..len]);
        println!("Received from {}: {}", addr, message);

        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file_path)
            .await
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
                continue;
            }
        };

        if let Err(e) = file.write_all(message.as_bytes()).await {
            eprintln!("Failed to write message to file: {}", e);
        }

        if let Err(e) = file.write_all(b"\n").await {
            eprintln!("Failed to write newline to file: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::net::UdpSocket;
    use tokio::fs;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_receive_ais_messages() -> Result<(), Box<dyn Error>> {
        let bind_addr = "127.0.0.1:50000";
        let output_file_path = "test_output.txt";

        // Spawn the receiver in a separate task
        let receiver_handle = tokio::spawn(async move {
            if let Err(e) = receive_ais_messages(bind_addr, output_file_path).await {
                eprintln!("Receiver error: {}", e);
            }
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
        receiver_handle.abort();

        Ok(())
    }
}
