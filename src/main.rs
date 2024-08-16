use core::{str, time};

use std::{
    io::{self, prelude::*, BufReader}, str::FromStr, thread::sleep
};

use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}};

pub mod packets;
use packets::packet_base::*;

async fn process_socket(mut stream: TcpStream) {
    println!("Connected");
    let mut buf: [u8; 1] = [0];
    loop {
        stream.read(&mut buf).await.unwrap();
        for byte in buf {
            match byte {
                _ => println!("Unknow packet: {}", byte)
            }
        }
        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => "not string"
        };
        println!("{}",s);
        sleep(time::Duration::from_millis(100));
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:3493").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        process_socket(stream).await;
    }
}

