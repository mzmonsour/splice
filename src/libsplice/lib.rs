#![crate_id = "splice#0.1-pre"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate serialize;

use std::io::{TcpStream, IoResult};
use std::io::net::ip::{SocketAddr, Ipv4Addr};

pub use proto::{RequestId, Request, Response};

pub mod conf;
pub mod proto;

pub fn connect(addr: &str, port: u16) -> Result<(Upstream, Downstream), proto::NegotiateError> {
     match TcpStream::connect(addr, port) {
        Ok(mut stream) => {
            match proto::negotiate(&mut stream).err() {
                Some(e) => Err(e),
                None => Ok((
                    Upstream {
                        sock: stream.clone(),
                        send_id: 0,
                    },
                    Downstream {
                        sock: stream
                    }
                )),
            }
        },
        Err(..) => Err(proto::NegotiateFailed),
    }
}

pub struct Upstream {
    sock: TcpStream,
    send_id: RequestId,
}

impl Drop for Upstream {
    fn drop(&mut self) {
        self.sock.close_write();
    }
}

impl Upstream {
    pub fn send_request(&mut self, req: &Request) -> IoResult<RequestId> {
        try!(proto::send_request(&mut self.sock, self.send_id, req));
        let result = Ok(self.send_id);
        self.send_id += 1;
        result
    }
}

pub struct Downstream {
    sock: TcpStream
}

impl Drop for Downstream {
    fn drop(&mut self) {
        self.sock.close_read();
    }
}

impl Downstream {
    pub fn get_response(&mut self) -> IoResult<(RequestId, Response)> {
        let (proto::ResponseHeader {
            id: id,
            from: from,
            len: _,
        }, resp) = try!(proto::recv_response(&mut self.sock));
        Ok((from, resp))
    }
}

type Object = u64;

pub struct Buffer {
    path:   Path,
    id:     Object,
    start:  uint,
    buf:    Vec<String>
}

impl Buffer {
    pub fn apply_diff(&mut self, diff: &Differ) {
    }
}

pub struct Differ;
