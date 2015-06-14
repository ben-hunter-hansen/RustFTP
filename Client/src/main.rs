/*
    RustFTP Client
    main.rs
    6/13/15
    @author Ben Hansen
*/
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::{File};
use std::path::Path;
use std::error::Error;

fn get_input() -> String {
    let stdin = io::stdin();
    return stdin.lock().lines().next().unwrap().unwrap();
}

fn load_file_contents(file_path: String) -> Vec<u8> {
    let path = Path::new(&file_path);
    let path_str = path.display();
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            let desc = Error::description(&why);
            panic!("Error opening file at {}: {}", path_str, desc);
        }
    };

    let mut buf: Vec<u8> = Vec::new();
    let mut reader = std::io::BufReader::new(&file);
    println!("Reading file: {}, please wait.",path_str);
    reader.read_to_end(&mut buf).unwrap();
    println!("Finished reading file, {} bytes read.",buf.len());
    return buf;
}

fn send_packet(chunk: Vec<u8>,file_id: String) {
    let mut stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    stream.write_all(file_id.as_bytes()).unwrap();
    stream.write(b"\n").unwrap();
    stream.write(&chunk[..]).unwrap();
}

fn get_file_id(file_path: String) -> String {

    let extract_file_name = |delim: char| -> String {
        let split = file_path.split(delim).last();
        return String::from(split.unwrap());
    };

    let mut id = String::new();
    if file_path.contains("/") { // Linux path
        id = extract_file_name('/');
    } else if file_path.contains("\\") { // Windows path
        id = extract_file_name('\\');
    }

    return id;
}

fn main() {
    println!("Enter the full path of the file you wish to transmit.");
    let path = get_input();
    let file_contents = load_file_contents(path.clone());
    let identifier = get_file_id(path);
    send_packet(file_contents,identifier);
}
