KV-Lake is a caching server that stores key-value pairs of strings. Currently, supports LRU and LFU eviction modes.
This is my attempt to write some networking code and learn about cache eviction strategies while learning the rust programming language.

Available commands:
PUT key value
GET key
DEL key

Basic usage:
1. Just clone this repo
2. cd to the repo
3. run "cargo run"

and your server should be up and running. You will see output similar to the below output:
```
your-username:mykvstore yourusername$ cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.00s
Running `target/debug/mykvstore`
Using LRU eviction strategy
queue size: 5
Listening on port 8000
```
By default it picks the LRU eviction strategy with queue size of 5, listening on port 8000. You can override these values by passing your values to the `cargo run` command.
`cargo run PORT QUEUE_SIZE EVICTION_STRATEGY`
eg: `cargo run 4000 10 lfu`
Note: `lfu` and `lru` are the only valid values for `EVICTION_STRATEGY` as of now.

Use the  [kv-lake-cli](https://github.com/blackblood/kv-lake-cli) client to connect to this server
