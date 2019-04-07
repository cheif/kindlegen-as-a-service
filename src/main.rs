#[macro_use]
extern crate rouille;

use std::process::Command;
use std::fs;
use std::io::Read;
use rouille::Response;

fn main() {
    println!("Starting server");
    rouille::start_server("0.0.0.0:8080", move |request| {
        router!(request,
                (PUT) (/) => {
                    println!("Starting request");
                    let mut data = request.data().expect("Could not get body");

                    let mut buf = Vec::new();
                    match data.read_to_end(&mut buf) {
                        Ok(_) => (),
                        Err(_) => return Response::text("Failed to read body")
                    };

                    println!("... Writing to disk");
                    // Write the epub to disk, so that we can run kindlegen on it
                    fs::write("/tmp/file.epub", buf).expect("Unable to write file");

                    println!("... Running kindlegen");
                    Command::new("kindlegen")
                        .arg("/tmp/file.epub")
                        .output()
                        .expect("Error when running kindlegen");


                    println!("Returning to user");
                    Response::from_file("application/mobi", fs::File::open("/tmp/file.mobi").unwrap())
                },
                _ => Response::empty_404()
                )
    });
}
