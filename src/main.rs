pub mod stores;
use stores::lru::LRUCache;
use stores::lfu::LFUCache;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::{Arc, RwLock};
use std::thread;
use std::env;

enum Command<T> {
    PUT(String, T),
    GET(String),
    DEL(String),
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
    } else if c == "DEL" {
        Ok(Command::DEL(input_vec[1].to_string()))
    } else {
        Err("Received unknown command".to_string())
    }
}

fn handle_connection(conn: &mut TcpStream, cache: Arc<RwLock<dyn stores::Cacheable<String>>>) -> io::Result<String> {
    loop {
        let mut input_length = [0; 1];
        conn.read(&mut input_length)?;
        if input_length[0] == 0 {
            println!("shutting down. bye!");
            conn.shutdown(Shutdown::Both).expect("failed to shutdown");
            return Ok("".to_string());
        }
        let mut input_buffer: Vec<u8> = vec![0; input_length[0] as usize];
        conn.read(&mut input_buffer)?;
        let string_buffer = std::str::from_utf8(&input_buffer).unwrap();
        println!("Received Command: {}", string_buffer);
        match get_command(string_buffer) {
            Ok(command) => match command {
                Command::PUT(key, value) => {
                    let mut m_cache = cache.write().unwrap();
                    m_cache.put(key.to_string(), String::clone(&value));
                    let output = format!("Added {}: {}", key, value);
                    conn.write(&[output.len() as u8]).expect("len socket write failed");
                    conn.write(output.as_bytes()).expect("data socket write failed");
                    println!("hashmap-----------");
                    m_cache.print_map();
                    println!("linked list-----------");
                    m_cache.print_list();
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
                    println!("hashmap-----------");
                    m_cache.print_map();
                    println!("linked list-----------");
                    m_cache.print_list();
                }
                Command::DEL(key) => {
                    let mut m_cache = cache.write().unwrap();
                    let mut output = String::new();
                    if let Err(msg) = m_cache.delete(key) {
                        output = msg;
                    }
                    println!("output = {}", output);
                    conn.write(&[output.len() as u8]).expect("len socket write failed");
                    conn.write(output.as_bytes()).expect("data socket write failed");
                    println!("hashmap-----------");
                    m_cache.print_map();
                    println!("linked list-----------");
                    m_cache.print_list();
                }
                Command::QUIT => {
                    println!("shutting down. bye!");
                    conn.shutdown(Shutdown::Both).expect("failed to shutdown");
                    return Ok("".to_string());
                }
            },
            Err(_e) => {
                let output = "unknown command";
                println!("{}", output);
                conn.write(&[output.len() as u8]).expect("len socket write failed");
                conn.write(output.as_bytes()).expect("data socket write failed");
            },
        };
    }
}

fn get_port_number(args: &mut Vec<String>) -> u32 {
    if args.is_empty() {
        println!("defaulting to port: 8000");
        8000
    } else {
        let port = match args[0].to_string().parse::<u32>() {
            Ok(p) => p,
            Err(_msg) => {
                println!("invalid port number. defaulting to 8000");
                8000
            }
        };
        args.remove(0);
        port
    }
}

fn get_queue_size(args: &mut Vec<String>) -> u32 {
    if args.is_empty() {
        println!("defaulting to queue size: 5");
        5
    } else {
        let queue_size = match args[0].to_string().parse::<u32>() {
            Ok(qs) => qs,
            Err(_msg) => {
                println!("invalid queue size. defaulting to queue size: 5");
                5
            }
        };
        args.remove(0);
        queue_size
    }
}

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut port = 8000;
    let mut queue_size = 5;
    let mut eviction_strat = "lru";
    if !args.is_empty() {
        port = get_port_number(&mut args);
        queue_size = get_queue_size(&mut args);
        eviction_strat = if args.is_empty() {
            println!("defaulting to lru eviction strategy");
            "lru"
        } else {
            args[0].as_str()
        };
    }

    let cache_ptr: Arc<RwLock<dyn stores::Cacheable<String> + std::marker::Send + std::marker::Sync>> = if &eviction_strat == &"lfu" {
        println!("Using LFU eviction strategy");
        Arc::new(RwLock::new(LFUCache::new(queue_size)))
    } else {
        println!("Using LRU eviction strategy");
        Arc::new(RwLock::new(LRUCache::new(queue_size)))
    };
    println!("queue size: {}", queue_size);

    let conn = TcpListener::bind(format!("localhost:{}", port))?;
    println!("Listening on port {}", port);
    for stream in conn.incoming() {
        let mut sock = stream.unwrap().try_clone().unwrap();
        let cache_ref = Arc::clone(&cache_ptr);
        thread::spawn(move || {
            handle_connection(&mut sock, cache_ref).expect("Handle connection failed.");
        });
    }
    Ok(())
}
