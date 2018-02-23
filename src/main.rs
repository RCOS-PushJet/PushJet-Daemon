extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;
extern crate uuid;

use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;


fn main() {
    let mut core = Core::new().unwrap();
    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());
    let mut file = File::open("keys/public").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut file = File::open("keys/uuid").unwrap();
    let mut uuid = String::new();
    file.read_to_string(&mut uuid).unwrap();

    let url = String::from("https://api.pushjet.io/service?service=") + &contents;
    let uri = url.parse::<hyper::Uri>().unwrap();
    println!("url: {:?}", uri);

    let work = client.get(uri).and_then(|res| {
        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    });
    core.run(work).unwrap();

    let mut stream = TcpStream::connect("api.pushjet.io:7171").unwrap();

    let _ = stream.set_read_timeout(Some(Duration::new(5, 0)));
    let _ = stream.set_nodelay(true);
    // ignore the Result
    let mut thing: [u8; 150] = [0; 150];
    println!("{:?}", stream.write(uuid.as_bytes()));
    println!("AAAA");
    let sizein = stream.read(&mut thing).unwrap();
    println!("BBBBBBBb");
    println!("{:?}", String::from_utf8_lossy(&thing[1..sizein - 1]));
    let mut i = stream.read(&mut thing);
    while i.is_err() {
        println!("timeout {:?}", i);
        i = stream.read(&mut thing);
        println!("write {:?}", stream.write(uuid.as_bytes()));
    }
    println!("{:?}", String::from_utf8_lossy(&thing[1..sizein - 1]));
}
