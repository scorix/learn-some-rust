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
    println!("hash: {:?}", value);
}

fn get_a_set(con: &redis::Connection, key: &str) {
    let value : redis::Value = redis::cmd("SRANDMEMBER").arg(key).query(con).unwrap();
    println!("got a random value: {:?} from set: {:?}", value, key);
}

fn main() {
    let con : redis::Connection = establish_connection();
    // This will raise an error while compiling:
    //   redis::cmd("FLUSHALL").query(&con).unwrap();
    //
    // error: unable to infer enough type information about `_`; type annotations or generic parameter binding required [E0282]
    //
    // It is the same as
    //   redis::cmd("FLUSHALL").query::<_>(&con).unwrap();
    redis::cmd("FLUSHALL").query::<()>(&con).unwrap();
    let x : String = con.set::<&str, i32, _>("a", 43).unwrap();
    println!("Redis says: {:?}", x);
    con.set::<&str, &str, ()>("b", "foo").unwrap();

    get(&con, "a");
    get(&con, "b");
    get_a_random_key(&con);
    get_a_missing_key(&con, "missing_key");

    redis::cmd("HMSET").arg("hash_key").arg("foo").arg("bar").arg("size").arg(1).query::<()>(&con).unwrap();
    get_a_hash_key(&con, "hash_key");
    get_a_hash_key(&con, "missing_hash");

    redis::cmd("SADD").arg("set").arg("foo").arg("bar").query::<()>(&con).unwrap();
    get_a_set(&con, "set")
}
