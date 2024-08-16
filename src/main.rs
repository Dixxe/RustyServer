use core::{panic, str, time};

use std::{
    io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, os::fd::AsFd, thread::sleep
};

pub mod packets;
use packets::packet_base::*;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:3493").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("connection!");
        handle_connection(&stream);
        game_input_stream(&stream);
    }
}

fn handle_connection(stream: &TcpStream) {
    let buf_reader = BufReader::new(stream);
    dbg!(&buf_reader);

    
}

fn game_input_stream(mut stream: &TcpStream) {
    let mut buf: [u8; 1] = [0];
    loop {
        stream.read(&mut buf[..]).unwrap(); // ОНО ДАЕТ ПАКЕТЫ ХАХАХЫХВЫВ
        for byte in buf {
            match byte {
                _ => print!("Unknown packet: {} \n", byte)
            }
        }
        let s = match  str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => "wrong",
        };
        println!("{}", s);
        sleep(time::Duration::from_millis(100));
    }
}

fn game_output_strean(mut stream: &TcpStream, byte: u8) {
    let mut buf = [byte];
    stream.write_all(&buf).unwrap();
}
