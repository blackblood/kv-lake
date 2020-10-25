use mykvstore::LRUCache;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

enum Command {
    PUT(String, String),
    GET(String),
    QUIT,
}

fn get_command(raw_input: &str) -> Result<Command, String> {
    let input_vec: Vec<&str> = raw_input.split(" ").collect();
    let c = input_vec[0];
    if c == "PUT" {
        Ok(Command::PUT(
            input_vec[1].to_string(),
            input_vec[2].to_string(),
        ))
    } else if c == "GET" {
        Ok(Command::GET(input_vec[1].to_string()))
    } else if c == "quit" {
        Ok(Command::QUIT)
    } else {
        Err("Received unknown command".to_string())
    }
}

fn handle_connection(conn: &mut TcpStream, cache: &mut Arc<Mutex<LRUCache<String>>>) -> io::Result<(String)> {
    let mut input_length = [0; 1];
    conn.read(&mut input_length)?;
    let mut input_buffer: Vec<u8> = vec![0; input_length[0] as usize];
    conn.read(&mut input_buffer)?;
    let string_buffer = std::str::from_utf8(&input_buffer).unwrap();
    println!("Received Command: {}", string_buffer);
    match get_command(string_buffer) {
        Ok(command) => match command {
            Command::PUT(key, value) => {
                cache.lock().unwrap().put(key.to_string(), value.to_string());
                println!("Added {}: {}", key, value);
                return Ok(format!("Added {}: {}", key, value));
            }
            Command::GET(key) => {
                if let Some(res) = cache.lock().unwrap().get(key) {
                    return Ok(res.to_string());
                }
            }
            Command::QUIT => {
                return Ok(format!("quit ack"));
            }
        },
        Err(_e) => (),
    };
    Ok("".to_string())
}

fn main() -> io::Result<()> {
    let lru_cache_ptr = Arc::new(Mutex::new(LRUCache::new()));
    let conn = TcpListener::bind("localhost:8000")?;
    println!("Listening on port 8000");
    for stream in conn.incoming() {
        let mut sock = stream.unwrap().try_clone().unwrap();
        let mut lru_cache = Arc::clone(&lru_cache_ptr);
        thread::spawn(move || {
            loop {
                let output = handle_connection(&mut sock, &mut lru_cache).expect("Handle connection failed.");
                println!("output = {}", output);
                sock.write(&[output.len() as u8]).expect("len socket write failed");
                sock.write(output.as_bytes()).expect("data socket write failed");
                for (k, v) in lru_cache.lock().unwrap().map.iter() {
                    println!("{}: {}", k, v);
                }
                if output == "quit ack" {
                    break;
                }
            }
        });
    }
    Ok(())
}
