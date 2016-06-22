extern crate redis;
use redis::{Commands};

// connect to redis
fn establish_connection() -> redis::Connection {
    let client = redis::Client::open("redis://192.168.203.135:31066/0").unwrap();
    client.get_connection().unwrap()
}

fn set_and_get<T: redis::ToRedisArgs>(con: &redis::Connection, key: &str, value: T) {
    let _ : () = con.set(key, value).unwrap();
    let value : redis::Value = con.get(key).unwrap();
    let ttl : i64 = redis::cmd("TTL").arg(key).query(con).unwrap();
    println!("key: {k:?}, value: {v:?}, ttl: {ttl:?}", k = key, v = value, ttl = ttl);
}

fn get_a_random_key(con: &redis::Connection) {
    let random_key : String = redis::cmd("RANDOMKEY").query(con).unwrap();
    let value : String = con.get(random_key.to_string()).unwrap();
    println!("random key is {}, its value is {}.", random_key, value);
}

fn main() {
    let con : redis::Connection = establish_connection();
    let _ : () = redis::cmd("FLUSHALL").query(&con).unwrap();
    set_and_get(&con, "a", 43);
    set_and_get(&con, "b", "foo");
    get_a_random_key(&con);
}
