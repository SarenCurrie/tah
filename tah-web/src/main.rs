#[macro_use]
extern crate rouille;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rusqlite;

use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

use rouille::Request;
use rouille::Response;

use rusqlite::Connection;

#[derive(Serialize, Debug)]
struct Entry {
    timestamp: String,
    reading: f64,
    reading_type: String,
    location: String
}

fn main() {
    println!("Now listening on localhost:8000");

    let conn = Mutex::new(Connection::open(Path::new("../data/tah.sqlite")).unwrap());

    conn.lock().unwrap().execute("CREATE TABLE IF NOT EXISTS entry (
                  id       INTEGER PRIMARY KEY,
                  time     TEXT NOT NULL,
                  reading  FLOAT NOT NULL,
                  type     TEXT NOT NULL,
                  location TEXT NOT NULL
                  )", &[]).unwrap();

    rouille::start_server("localhost:8000", move |request| {
        let response = rouille::match_assets(&request, "static");
        if response.is_success() {
            return response;
        }
        let db = conn.lock().unwrap();
        routes(&request, &db)
    });
}

fn routes(request: &Request, db: &Connection) -> Response {
    router!(request,
        (GET) (/) => {
            rouille::match_assets(&request, "static/index.html")
        },

        (GET) (/history) => {
            let mut out = Vec::new();
            let mut stmt = db.prepare("SELECT time, reading, type, location FROM entry").unwrap();
            let entry_iter = stmt.query_map(&[], |row| {
                Entry {
                    timestamp: row.get(0),
                    reading: row.get(1),
                    reading_type: row.get(2),
                    location: row.get(3)
                }
            }).unwrap();
            for entry in entry_iter {
                out.push(entry.expect("Something broke"))
            }
            println!("{:?}", out);

            Response::json(&out)
        },

        // The code block is called if none of the other blocks matches the request.
        // We return an empty response with a 404 status code.
        _ => rouille::Response::empty_404()
    )
}
