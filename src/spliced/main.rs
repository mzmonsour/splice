#![crate_id = "spliced#0.1-pre"]
#![license = "MIT"]
#![crate_type = "bin"]

extern crate green;
extern crate rustuv;
extern crate serialize;
extern crate splice;

use std::comm::{Disconnected, Empty};
use proto::{KeepAlive, OpenBuffer, OpenFile, RunCommand};
use proto::{Ack, Allow, Deny, Handle};
use proto::{RequestId, Request};

mod conn;
mod proto;

#[start]
fn start(argc: int, argv: **u8) -> int {
    green::start(argc, argv, rustuv::event_loop, main)
}

fn main() {
    let cfg = splice::conf::load_default().unwrap();
    let mut server = conn::Server::new(
        cfg.default_server_addr.as_slice(), cfg.default_server_port, None).unwrap();
    loop {
        let raw_client = server.accept();
        spawn(proc() {
            match raw_client.ok() {
                Some(c) => {
                    match c.negotiate().ok() {
                        Some((down, mut up)) => {
                            println!("Negotiated");
                            let (tx, rx) = channel::<(RequestId, Request)>();
                            spawn(proc() {
                                let mut down = down;
                                'resp: loop {
                                    match rx.try_recv() {
                                        Ok((id, req)) => {
                                            match down.send_response(id, &match req {
                                                KeepAlive => Ack,
                                                _ => Deny, // Deny unimplemented requests
                                            }) {
                                                _ => (), // Ignore errors for now
                                            }
                                        },
                                        Err(Disconnected) => break 'resp,
                                        Err(Empty) => (),
                                    };
                                }
                            });
                            'req: loop {
                                match up.get_request() {
                                    Ok(v) => match tx.send_opt(v) {
                                        Err(..) => break 'req,
                                        _ => (),
                                    },
                                    Err(e) => match e.kind {
                                        std::io::Closed => break 'req,
                                        _ => (),
                                    },
                                };
                            }
                        }
                        None => println!("Negotiate failed"),
                    }
                },
                None => println!("No connection"),
            }
        });
    }
}
