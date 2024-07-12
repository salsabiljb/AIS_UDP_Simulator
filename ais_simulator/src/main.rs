mod sender;
mod receiver;

use std::error::Error;
use tokio::join;
use clap::Parser;

/// AIS UDP Streamer
#[derive(Parser)]
struct Args {
    /// Path to the file containing AIS messages
    #[clap(long)]
    sender_file_path: String,

    /// Target address to send UDP messages
    #[clap(long)]
    target_addr: String,

    /// Address to bind the receiver (optional)
    #[clap(long)]
    bind_addr: Option<String>,

    /// Path to the file where received messages will be saved (optional)
    #[clap(long)]
    receiver_file_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let sender_task = sender::send_ais_messages(&args.sender_file_path, &args.target_addr);

    if let (Some(bind_addr), Some(receiver_file_path)) = (args.bind_addr, args.receiver_file_path) {
        let receiver_task = receiver::receive_ais_messages(&bind_addr, &receiver_file_path);
        let (sender_result, receiver_result) = join!(sender_task, receiver_task);
        sender_result?;
        receiver_result?;
    } else {
        sender_task.await?;
    }

    Ok(())
}
