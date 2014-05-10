#![crate_id = "splice#0.1-pre"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

use std::io::{TcpStream, IoResult};
use std::io::net::ip::{SocketAddr, Ipv4Addr};

pub fn connect(addr: SocketAddr) -> IoResult<(Upstream, Downstream)> {
    let stream = try!(TcpStream::connect(addr));
    Ok((
        Upstream {
            sock: stream.clone()
        },
        Downstream {
            sock: stream
        }
    ))
}

pub struct Upstream {
    sock: TcpStream
}

impl Drop for Upstream {
    fn drop(&mut self) {
        self.sock.close_write();
    }
}

impl Upstream {
    pub fn send_request(&self, req: &Request) -> IoResult<()> {
        Ok(())
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
    pub fn get_response(&self) -> Option<Response> {
        None
    }
}

type Object = uint;

pub enum Response {
    BufferOpened(Object, Buffer),
    BufferChanged(Object, Differ),
    ConnectionClosed,
    Empty,
    Unknown
}

pub struct Buffer {
    path:   ~str,
    id:     Object,
    start:  uint,
    buf:    Vec<StrBuf>
}

impl Buffer {
    pub fn apply_diff(&mut self, diff: &Differ) {
    }
}

pub struct Differ;
pub struct Request;
