use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::error::Error;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UdpSocket;
use tokio::time::{self, Duration};

pub async fn send_ais_messages(
    file_path: &str,
    target_addr: &str,
) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path).await?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    let mut lines_stream = reader.lines();
    while let Some(line) = lines_stream.next_line().await? {
        lines.push(line);
    }

    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    let mut rng = StdRng::from_entropy();
    for _ in 0..5 {
        lines.shuffle(&mut rng);
        for line in &lines {
            let message = line.trim();
            if !message.is_empty() {
                socket.send_to(message.as_bytes(), target_addr).await?;
                println!("{}", message);
            }
            let delay = Duration::from_millis(100 + rng.gen_range(0..1000));
            time::sleep(delay).await;
        }
    }

    Ok("ok".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::sync::Arc;
    use tokio::io::AsyncWriteExt;
    use tokio::sync::Mutex;
    use tokio::time::{sleep, Duration};

    async fn create_test_file(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(file_path).await?;
        file.write_all(content.as_bytes()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_send_ais_messages() -> Result<(), Box<dyn Error>> {
        let file_path = "test_input.txt";
        let target_addr = "127.0.0.1:12345";

        create_test_file(file_path, "message1\nmessage2\nmessage3\n").await?;
        let bind_addr = "127.0.0.1:12345";
        let socket = tokio::net::UdpSocket::bind(bind_addr).await?;
        let messages_received = Arc::new(Mutex::new(Vec::new()));

        let listener_handle = {
            let messages_received = Arc::clone(&messages_received);
            tokio::spawn(async move {
                let mut buf = vec![0; 1024];
                while let Ok((len, _)) = socket.recv_from(&mut buf).await {
                    let message = String::from_utf8_lossy(&buf[..len]);
                    messages_received.lock().await.push(message.to_string());
                }
            })
        };

        sleep(Duration::from_millis(100)).await;

        if let Err(e) = send_ais_messages(file_path, target_addr).await {
            eprintln!("Failed to send AIS messages: {}", e);
        }

        sleep(Duration::from_millis(100)).await;

        let received_messages = messages_received.lock().await;
        assert!(received_messages.contains(&"message1".to_string()));
        assert!(received_messages.contains(&"message2".to_string()));
        assert!(received_messages.contains(&"message3".to_string()));

        // Clean up
        tokio::fs::remove_file(file_path).await?;
        listener_handle.abort();

        Ok(())
    }
}
