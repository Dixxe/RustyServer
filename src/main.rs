use core::time;

use std::{
    io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, os::fd::AsFd, thread::sleep
};

pub mod packets;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3493").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("connection!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    dbg!(&buf_reader);

    
    let mut tick_count: i32 = 0;
    let mut buf = [0, 10];
    loop {
        // stream.write_all(&[0]);
        stream.read(&mut buf[..]);
        sleep(time::Duration::from_millis(500));
        println!("tick {}!\n-------------------------------", tick_count);
        dbg!(buf); // ОНО ДАЕТ ПАКЕТЫ ХАХАХЫХВЫВ
        tick_count += 1;
    }
    
}
