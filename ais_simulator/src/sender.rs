use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UdpSocket;
use tokio::time::{self, Duration};
use std::error::Error;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub async fn send_ais_messages(file_path: &str, target_addr: &str) -> Result<String, Box<dyn Error>> {
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
            let delay = Duration::from_millis(100 + rng.gen_range(0..2000));
            time::sleep(delay).await;
        }
    }

    Ok("ok".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
    use tokio::time::{sleep, Duration};

    async fn create_test_file(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
        println!("create test");
        let mut file = File::create(file_path).await?;
        file.write_all(content.as_bytes()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_send_ais_messages() -> Result<(), Box<dyn Error>> {
        let file_path = "test_input.txt";
        let target_addr = "192.168.1.74:12345";
        println!("hereee");
        create_test_file(file_path, "message1\nmessage2\nmessage3\n").await?;
        let bind_addr = "192.168.1.74:12345";
        let socket = tokio::net::UdpSocket::bind(bind_addr).await?;
        let listener_handle = tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            while let Ok((len, addr)) = socket.recv_from(&mut buf).await {
                println!("Test received from {}: {}", addr, String::from_utf8_lossy(&buf[..len]));
            }
        });

        sleep(Duration::from_millis(100)).await;

        send_ais_messages(file_path, target_addr).await?;

        // fs::remove_file(file_path).await?;

        listener_handle.abort();

        Ok(())
    }
}
