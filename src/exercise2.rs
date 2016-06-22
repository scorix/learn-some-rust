extern crate redis;
use redis::{Commands};
use std::collections::HashMap;

// connect to redis
fn establish_connection() -> redis::Connection {
    let client = redis::Client::open("redis://192.168.203.135:31066/0").unwrap();
    client.get_connection().unwrap()
}

fn get(con: &redis::Connection, key: &str) {
    let value : Option<redis::Value> = con.get(key).unwrap();
    let ttl : i64 = redis::cmd("TTL").arg(key).query(con).unwrap();
    println!("key: {k:?}, value: {v:?}, ttl: {ttl:?}", k = key, v = value, ttl = ttl);
}

fn get_a_random_key(con: &redis::Connection) {
    let random_key : String = redis::cmd("RANDOMKEY").query(con).unwrap();
    let value : String = con.get(random_key.to_string()).unwrap();
    println!("random key is {}, its value is {}.", random_key, value);
}

fn get_a_missing_key(con: &redis::Connection, key: &str) {
    let value : Option<redis::Value> = con.get(key).unwrap();
    println!("key: {k:?}, value: {v:?}", k = key, v = value);
}

fn get_a_hash_key(con: &redis::Connection, key: &str) {
    let value : HashMap<String, redis::Value> = redis::cmd("HGETALL").arg(key).query(con).unwrap();
    println!("{:?}", value);
}

fn get_a_set(con: &redis::Connection) {
    unimplemented!()
}

fn main() {
    let con : redis::Connection = establish_connection();
    let _ : () = redis::cmd("FLUSHALL").query(&con).unwrap();
    let _ : () = con.set("a", 43).unwrap();
    let _ : () = con.set("b", "foo").unwrap();

    get(&con, "a");
    get(&con, "b");
    get_a_random_key(&con);
    get_a_missing_key(&con, "missing_key");

    let _ : () = redis::cmd("HMSET").arg("hash_key").arg("foo").arg("bar").arg("size").arg(1).query(&con).unwrap();
    get_a_hash_key(&con, "hash_key");
    get_a_hash_key(&con, "missing_hash");
}
