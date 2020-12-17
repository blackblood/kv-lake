// pub mod stores;
use mykvstore::stores::lru::LRUCache;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

enum Command<T> {
    PUT(String, T),
    GET(String),
    QUIT,
}

fn get_command(raw_input: &str) -> Result<Command<String>, String> {
    let mut raw_input = raw_input.to_string();
    raw_input.pop(); // removing trailing newline char
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

fn handle_connection(conn: &mut TcpStream, cache: Arc<RwLock<LRUCache<String>>>) -> io::Result<String> {
    loop {
        let mut input_length = [0; 1];
        conn.read(&mut input_length)?;
        let mut input_buffer: Vec<u8> = vec![0; input_length[0] as usize];
        conn.read(&mut input_buffer)?;
        let string_buffer = std::str::from_utf8(&input_buffer).unwrap();
        println!("Received Command: {}", string_buffer);
        match get_command(string_buffer) {
            Ok(command) => match command {
                Command::PUT(key, value) => {
                    let mut m_cache = cache.write().unwrap();
                    m_cache.put(key.to_string(), String::clone(&value));
                    m_cache.print_map();
                    let output = format!("Added {}: {}", key, value);
                    conn.write(&[output.len() as u8]).expect("len socket write failed");
                    conn.write(output.as_bytes()).expect("data socket write failed");
                }
                Command::GET(key) => {
                    let mut m_cache = cache.write().unwrap();
                    if let Some(output) = m_cache.get(key) {
                        println!("output = {}", output);
                        conn.write(&[output.len() as u8]).expect("len socket write failed");
                        conn.write(output.as_bytes()).expect("data socket write failed");
                    } else {
                        println!("Not found in cache");
                    }
                    m_cache.print_list();
                }
                Command::QUIT => {
                    println!("quit ack");
                    return Ok("".to_string());
                }
            },
            Err(_e) => (),
        };
    }
}

fn main() -> io::Result<()> {
    let lru_cache_ptr = Arc::new(RwLock::new(LRUCache::new()));
    let conn = TcpListener::bind("localhost:8000")?;
    println!("Listening on port 8000");
    for stream in conn.incoming() {
        let mut sock = stream.unwrap().try_clone().unwrap();
        let lru_cache_ref = Arc::clone(&lru_cache_ptr);
        thread::spawn(move || {
            handle_connection(&mut sock, lru_cache_ref).expect("Handle connection failed.");
        });
    }
    Ok(())
}
