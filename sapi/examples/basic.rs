use slog::{error, info, o, Drain, Logger};
use std::sync::Mutex;
use joyent_rust_utils::net;

fn main() {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let log = Logger::root(
        Mutex::new(slog_term::FullFormat::new(plain).build()).fuse(),
        o!("build-id" => "0.1.0"),
    );

    let sapi_ip = net::lookup_ip("sapi.ruidc0.joyent.us").unwrap();
    let sapi_url = format!("http://{}", sapi_ip);
    let client = sapi::SAPI::new(&sapi_url, 60, log.clone());
    let zone_uuid = String::from("f8bf03e3-5636-4cc4-a939-bbca6b4547f0");

    match client.get_zone_config(&zone_uuid) {
        Ok(resp) => {
            info!(log, "config: {:?}", resp);
        }
        Err(e) => error!(log, "error: {:?}", e),
    }
}
