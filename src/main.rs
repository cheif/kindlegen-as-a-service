#[macro_use]
extern crate rouille;
extern crate uuid;

use std::fs;
use std::io::Read;
use rouille::Request;
use rouille::Response;
mod kindle;

fn main() {
    println!("Starting server");
    rouille::start_server("0.0.0.0:8080", move |request| {
        router!(request,
                (PUT) (/) => {
                    match process_request(request) {
                        Ok(file) => Response::from_file("application/mobi", file),
                        Err(error) => Response::text(error).with_status_code(400)
                    }
                },
                _ => Response::empty_404()
                )
    });
}

fn process_request(request: &Request) -> Result<fs::File, String> {
    println!("Starting request");
    let mut data = request.data().ok_or_else(|| "Could not get body")?;

    let mut buf = Vec::new();
    match data.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read body".to_string())
    };

    return kindle::convert(buf);
}
