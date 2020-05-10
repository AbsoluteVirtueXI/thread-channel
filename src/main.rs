use tokio::sync::{mpsc,Mutex};
use tokio::time::{interval, Duration, delay_for};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let (mut tx, mut rx) = mpsc::channel(100);
    let rx1 = Arc::new(Mutex::new(rx));
    let rx2 = Arc::clone(&rx1);

    let mut interval = interval(Duration::from_secs(1));

    // Every 1 second we send a message to the receiver half of the channel
    let ticker = tokio::spawn(async move {
        loop {
            interval.tick().await;
            if let Err(_) = tx.send("tick").await {
                println!("receiver dropped");
            }
        }
    });

    let ringer1 = tokio::spawn(async move {
        while let Some(i) = rx1.lock().await.recv().await {
            println!("Ringer 1: got = {}", i);
        }
    });

    // This taks is delayed by 2 seconds
    let ringer2 = tokio::spawn(async move {
        loop {
            delay_for(Duration::from_secs(2)).await;
            let data = rx2.lock().await.recv().await;
            println!("Ringer 2: got = {}", data.unwrap());
        }
    });

    ticker.await?;
    ringer1.await?;
    ringer2.await?;
    Ok(())
}

