KV-Lake is a caching server that stores key-value pairs of strings. Currently, supports LRU and LFU eviction modes.
This is my attempt to write some networking code and learn about cache eviction strategies while learning the rust programming language.

Basic usage:
1. Clone the repository
2. cd to the repository and "cargo run". This will start the server.

Code to connect to the server:
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut writer = TcpStream::connect("localhost:8000")?;
    let mut reader = writer.try_clone()?;

    loop {
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str)?;
        println!("Number of bytes: {}", input_str.len());
        if let Ok(n) = writer.write(&[input_str.len() as u8]) {
            println!("Length of input: {}", n);
        }
        if let Ok(n) = writer.write(input_str.as_bytes()) {
            println!("Ok(n) = {}", n);
            writer.flush()?;
        }
        let mut input_length = [0; 1];
        reader.read(&mut input_length)?;
        let mut input_buffer: Vec<u8> = vec![0; input_length[0] as usize];
        reader.read(&mut input_buffer)?;
        let string_buffer = std::str::from_utf8(&input_buffer).unwrap();
        println!("string_buffer = {}", string_buffer);
        if string_buffer == "quit ack" {
            break;
        }
    }

    Ok(())
}

save the above code in a file kv-lake-cli.rs

compile and run the above program:
rustc kv-lake-cli.rs
./kv-lake-cli
PUT key1 value1
PUT key2 value2
PUT key3 value3
GET key2
GET key3
DEL key1

Available commands
PUT key value
GET key
DEL key
