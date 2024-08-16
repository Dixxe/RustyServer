use core::str;
use std::{
    io::{self}, sync::Arc,
};
use packets::packet_base::Packet;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::{mpsc, Mutex}};

pub mod packets;

async fn process_packets(mut send_tx: mpsc::Sender<Vec<u8>>, mut recieved_rx: mpsc::Receiver<Vec<u8>>) {
    // function where packets are processed.
    while let Some(packets) = recieved_rx.recv().await {
        dbg!(packets);
    }
}

struct GameInputStream {
    stream: Arc<Mutex<TcpStream>>
}
impl GameInputStream {
    async fn packets_reciever(self, tx: mpsc::Sender<Vec<u8>>) {
    
        // function that reads incoming packets from players
    
        tokio::spawn(async move {
            let mut stream = self.stream.lock().await;
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
    }
}

struct GameOutputStream {
    stream: Arc<Mutex<TcpStream>>
}
impl GameOutputStream {
    async fn packets_sender(self, mut send_rx: mpsc::Receiver<Vec<u8>>) {
        let mut stream = self.stream.lock().await;
        while let Some(packets) = send_rx.recv().await {
            for packet in packets {
                stream.write_all(&[packet]);
            }
        }
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
    let listener = TcpListener::bind("127.0.0.1:3493").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let stream = Arc::new(Mutex::new(socket));
        let game_in = GameInputStream { stream: stream.clone() };
        let game_out = GameOutputStream { stream: stream.clone() };

        let (recieved_tx, recieved_rx) = mpsc::channel::<Vec<u8>>(100);
        let (send_tx, send_rx) = mpsc::channel::<Vec<u8>>(100);

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
