extern crate rand;
use std::env;
use std::io::{Read, Write};
use std::io;
use std::str::from_utf8;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
mod funcs;
use funcs::*;

pub fn server() {
    let listener = TcpListener::bind("localhost:8888".to_string()).unwrap();
    println!("Server listening...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {handle_request(stream)});
            }
            Err(e) => {println!("Error: {}", e);}
        }
    }
    drop(listener);
}

fn handle_request(mut stream: TcpStream) {
    let mut hash = [0 as u8; 5]; 
    let mut key = [0 as u8; 10];
    let mut message = [0 as u8;50];

    while match stream.read(&mut hash) {
        Ok(_) => {
            stream.read(&mut key).unwrap();
            stream.read(&mut message).unwrap();
            let received_hash = from_utf8(&hash).unwrap();
            let received_key = from_utf8(&key).unwrap();
            let new_key = next_session_key(&received_hash,&received_key);
            let result = new_key.clone().into_bytes();
            stream.write(&result).unwrap();
            stream.write(&message).unwrap();
            true
        },
        Err(_) => {
            println!("Connection error with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn client() {
    let adres = "localhost:8888";

    match TcpStream::connect(adres) {
        Ok(mut stream) => {
            println!("You are connected to the server");
            let mut data = [0 as u8; 50];
            let mut rep = [0 as u8; 50];
           
           loop {
               let hash_str = get_hash_str();
               let session_key = get_session_key();
               let next_key = next_session_key(&hash_str, &session_key);
               println!("Your message: ");
               let mut message = String::new();
               io::stdin().read_line(&mut message).unwrap();
               stream.write(&hash_str.into_bytes()).unwrap();
               stream.write(&session_key.into_bytes()).unwrap();
               stream.write(&message.into_bytes()).unwrap();

               match stream.read(&mut data) {
                Ok(size) => {
                    stream.read(&mut rep).unwrap();
                       let received_key = from_utf8(&data[0..size]).unwrap();
                       let response = from_utf8(&rep).unwrap();

                       if received_key == next_key {
                           println!("Client key: {}, server key: {}", next_key, received_key);
                       } 
                       else {break;}
                       println!("Response: {}", response);
                   }, 
                   Err(e) => {println!("Failed to receive data: {}", e);}
               }
           }
        }, 
        Err(e) => {println!("Failed to connect: {}", e);}
    }
}

fn main() {
	let params: Vec<String> = env::args().collect();
    if (params[1] == "ip:port") && (params[2] == "-n") {
        for _i in 0..params[3].parse().unwrap(){
            client();
        }   
    } 
    else if params[1] == "port" {
        server()
    }
    else {
        println!("Wrong parameters")
    }
}