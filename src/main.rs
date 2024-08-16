use core::{str, time};
use std::{
    io::{self, prelude::*}, str::FromStr, sync::Arc, thread::sleep,
};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::{mpsc, Mutex}};

pub mod packets;
use packets::packet_base::*;

async fn process_socket(stream: Arc<Mutex<TcpStream>>) {
    println!("Connected");

}

async fn game_input_stream(stream: Arc<Mutex<TcpStream>>) {
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(100); // Create a channel with a buffer of 100 packets

    // function that reads incoming packets from players

    let stream_clone = stream.clone();
    tokio::spawn(async move {
        let mut stream = stream_clone.lock().await;
        let mut buf = [0; 1024]; // packets saved here
        loop {
            match stream.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    let packets = buf[..n].to_vec();
                    if tx.send(packets).await.is_err() { // send incoming packets in mpsc
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from socket; err = {:?}", e);
                    break;
                }
            }
        }
    });

    
    while let Some(packet) = rx.recv().await { // wrap packets in processing functions
        tokio::spawn(async move {
            process_packet(packet).await;
        });
    }
}

async fn process_packet(packets: Vec<u8>) {
    dbg!(&packets);
    for packet in packets {
        println!("Packet:{} equals {}", packet, str::from_utf8(&[packet]).unwrap_or("NaN"));
    }

}

async fn game_output_stream(stream: Arc<Mutex<TcpStream>>, bytes: &[u8]) {
    loop {
        // TODO method. Use this to send packets to players.
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

        let stream_clone = stream.clone();
        tokio::spawn(async move {
            process_socket(stream_clone).await;
        });

        let stream_clone = stream.clone();
        tokio::spawn(async move {
            game_input_stream(stream_clone).await;
        });


    }
}
