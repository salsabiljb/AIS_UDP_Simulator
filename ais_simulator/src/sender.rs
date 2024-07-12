use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UdpSocket;
use tokio::time::{self, Duration};
use std::error::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub async fn send_ais_messages(file_path: &str, target_addr: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path).await?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    let mut lines_stream = reader.lines();
    while let Some(line) = lines_stream.next_line().await? {
        lines.push(line);
    }

    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    let mut rng = thread_rng();
    loop {
        lines.shuffle(&mut rng);
        for line in &lines {
            let message = line.trim();
            if !message.is_empty() {
                socket.send_to(message.as_bytes(), target_addr).await?;
                println!("{}", message);
            }
            let delay = Duration::from_micros(10000 + rng.gen_range(0..100000));
            time::sleep(delay).await;
        }
    }
}
