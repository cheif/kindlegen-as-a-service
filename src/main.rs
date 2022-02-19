#[macro_use]
extern crate rouille;
extern crate uuid;
extern crate log;
extern crate simple_logger;
extern crate infer;

use std::fs;
use std::io::Read;
use rouille::Request;
use rouille::Response;
use log::{info, debug, error};
use simple_logger::SimpleLogger;
mod kindle;

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Starting server");
    rouille::start_server("0.0.0.0:8080", move |request| {
        router!(request,
                (PUT) (/) => {
                    match process_request(request) {
                        Ok(file) => Response::from_file("application/mobi", file),
                        Err(error) => {
                            error!("Error when processing request: {:?}", error);
                            return Response::text(error).with_status_code(400)
                        }
                    }
                },
                _ => Response::empty_404()
                )
    });
}

fn process_request(request: &Request) -> Result<fs::File, String> {
    info!("Received request");
    debug!("Headers: {:?}", request.headers());
    let mut data = request.data().ok_or_else(|| "Could not get body")?;

    let mut buf = Vec::new();
    match data.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read body".to_string())
    };

    return kindle::convert(buf);
}
