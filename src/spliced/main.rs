#![crate_id = "spliced#0.1-pre"]
#![license = "MIT"]
#![crate_type = "bin"]

extern crate green;
extern crate rustuv;
extern crate splice;

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
                        Some((down, up)) => {
                            println!("Negotiated");
                        }
                        None => println!("Negotiate failed"),
                    }
                },
                None => println!("No connection"),
            }
        });
    }
}
