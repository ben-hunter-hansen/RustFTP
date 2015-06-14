/*
    RustFTP Server
    main.rs
    6/11/15
    @author Ben Hansen
*/
use std::io::{BufRead,BufReader,Write, Read};
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;


fn write_to_file(path: &Path, bytes: Vec<u8>) {
    let s = &*bytes;
    let mut f = File::create(path).unwrap();
    f.write_all(s).unwrap();
}

fn handle_client(stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    println!("Got connection from {}", addr);

    let mut writer = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut data: Vec<u8> = Vec::new();

    let mut file_name_buf = String::new();
    reader.read_line(&mut file_name_buf).unwrap();

    let mut path = PathBuf::from("/home/ben/");
    path.push(file_name_buf);

    let p: &Path = path.as_path();
    reader.read_to_end(&mut data).unwrap();

    println!("gots the datas...{}",data.len());
    write_to_file(p,data);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {
            handle_client(stream.unwrap());
        });
    }
}
