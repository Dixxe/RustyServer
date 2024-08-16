use core::{str, time};
use std::{
    any::Any, io::{self}, sync::Arc
};
use packets::packet_base::{pre_register, PACKET_PREREGISTER_CONNECTION};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::{mpsc, Mutex}, time::sleep};

pub mod packets;

async fn process_packets(mut send_tx: mpsc::Sender<Vec<u8>>, mut recieved_rx: mpsc::Receiver<Vec<u8>>) {
    // function where packets are processed.
    println!("Connected!");
    while let Some(packets) = recieved_rx.recv().await {
        for packet in packets {
            match packet {
                PACKET_PREREGISTER_CONNECTION => {
                    println!("connection?");
                    match send_tx.send(pre_register()).await{
                        Ok(n) => println!("sended to sender"),
                        Err(e) => println!("Error! {}", e)
                    };

                }
                _ => println!("unknown packet {}: {}", packet, str::from_utf8(&[packet]).unwrap_or("nan"))
            }
        }
    }
}

struct GameInputStream {
    read_half: Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedReadHalf>>,
}

impl GameInputStream {
    async fn packets_reciever(self, tx: mpsc::Sender<Vec<u8>>) {
        tokio::spawn(async move {
            let mut buf = [0; 1024]; // Buffer to store incoming packets
            let mut read_half = self.read_half.lock().await;
            loop {
                match read_half.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let packets = buf[..n].to_vec();
                        if tx.send(packets).await.is_err() { // Send incoming packets to the mpsc
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
    }
}

struct GameOutputStream {
    write_half: Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>>,
}

impl GameOutputStream {
    async fn packets_sender(&self, mut send_rx: mpsc::Receiver<Vec<u8>>) {
        while let Some(packets) = send_rx.recv().await {
            println!("I'm sending!");
            dbg!(&packets);
            let mut write_half = self.write_half.lock().await;
            for packet in packets {
                match write_half.write_all(&[packet]).await {
                    Ok(_) => println!("Sent {}", packet),
                    Err(e) => panic!("Can't send {}!", packet),
                };
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3493").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        let (read_half, write_half) = socket.into_split();
        let read_half = Arc::new(Mutex::new(read_half));
        let write_half = Arc::new(Mutex::new(write_half));

        let game_in = GameInputStream { read_half };
        let game_out = GameOutputStream { write_half };

        let (recieved_tx, recieved_rx) = mpsc::channel::<Vec<u8>>(100);
        let (send_tx, send_rx) = mpsc::channel::<Vec<u8>>(1000);

        tokio::spawn(async move {
            game_in.packets_reciever(recieved_tx).await
        });

        tokio::spawn(async move {
            process_packets(send_tx,recieved_rx).await;
        });

        tokio::spawn(async move {
            game_out.packets_sender(send_rx).await;
        });

    }
}
