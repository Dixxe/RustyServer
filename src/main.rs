use core::{str, time};
use std::{
    io::{self, prelude::*}, str::FromStr, sync::Arc, thread::sleep,
};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::Mutex};

pub mod packets;
use packets::packet_base::*;

async fn process_socket(stream: Arc<Mutex<TcpStream>>) {
    println!("Connected");
    // This is where you can add logic for processing the connection
    // You could pass `stream` to other async functions or tasks
}

async fn game_input_stream(stream: Arc<Mutex<TcpStream>>) {
    let mut buf: [u8; 1] = [0];
    loop {
        let mut stream_locked = stream.lock().await;
        stream_locked.read(&mut buf).await.unwrap();
        drop(stream_locked); // Release the lock as soon as possible

        for byte in buf {
            match byte {
                _ => println!("Unknown packet: {}", byte),
            }
        }
        
        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => "not string",
        };
        println!("{}", s);

        sleep(time::Duration::from_millis(100));
    }
}

async fn game_output_stream(stream: Arc<Mutex<TcpStream>>, bytes: &[u8]) {
    loop {
        let mut stream_locked = stream.lock().await;
        stream_locked.write_all(bytes).await.unwrap();
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:3493").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let stream = Arc::new(Mutex::new(socket));

        // Spawn a new task to handle the connection
        let stream_clone = stream.clone();
        tokio::spawn(async move {
            process_socket(stream_clone).await;
        });

        // You can spawn another task for input handling
        let stream_clone = stream.clone();
        tokio::spawn(async move {
            game_input_stream(stream_clone).await;
        });


    }
}
