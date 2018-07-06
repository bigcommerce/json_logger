#[macro_use]
extern crate json_logger;
#[macro_use]
extern crate serde_json;

use json_logger::Logger;

fn main() {
    let mut log = Logger::new("stdout", ::std::io::stdout());
    json_info!(log, "baz", json!({"a":1, "b":2}));
    json_info!(log, "bar", json!({"a":3, "b":4}));
}