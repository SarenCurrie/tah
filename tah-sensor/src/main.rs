extern crate dht22_pi;
extern crate rusqlite;

use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

use rusqlite::Connection;

fn main() {
    let conn = Mutex::new(Connection::open(Path::new("../data/tah.sqlite")).unwrap());

    let res = dht22_pi::read(4);
    if res.is_ok() {
        let reading = res.ok().unwrap();
        conn.lock().unwrap().execute("INSERT INTO entry (time, reading, type, location)
                VALUES (?1, ?2, ?3, ?4)",
               &[&((SystemTime::now()).duration_since(UNIX_EPOCH).expect("?").as_secs() * 1000).to_string(), &f64::from(reading.temperature), &"temperature", &"router"]).unwrap();

        conn.lock().unwrap().execute("INSERT INTO entry (time, reading, type, location)
                VALUES (?1, ?2, ?3, ?4)",
              &[&((SystemTime::now()).duration_since(UNIX_EPOCH).expect("?").as_secs() * 1000).to_string(), &f64::from(reading.humidity), &"humidity", &"router"]).unwrap();

        println!("{:?}", reading);
    } else {
        eprintln!("Could not read from sensor, try again in 2 minutes");
    }
}

