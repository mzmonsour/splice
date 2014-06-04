extern crate splice;

use std::io::net::tcp::{TcpListener, TcpAcceptor, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::IoResult;

use proto;
pub use proto::{RequestId, Request, ResponseId, Response};

pub struct Server {
    acceptor: TcpAcceptor,
    auth: Option<proto::AuthMethod>,
}

impl Server {
    pub fn new(addr: &str, port: u16, auth: Option<proto::AuthMethod>) -> IoResult<Server> {
        let listener = try!(TcpListener::bind(addr, port));
        Ok(Server {
            acceptor: try!(listener.listen()),
            auth: auth,
        })
    }

    pub fn accept(&mut self) -> IoResult<FutureClient> {
        Ok(FutureClient {
            stream: try!(self.acceptor.accept()),
            auth: self.auth.clone(),
        })
    }
}

pub struct FutureClient {
    stream: TcpStream,
    auth: Option<proto::AuthMethod>,
}

impl FutureClient {
    pub fn negotiate(mut self) -> Result<(Downstream, Upstream), proto::NegotiateError> {
        try!(proto::negotiate(&mut self.stream, &self.auth));
        Ok((Downstream {
            stream: self.stream.clone(),
            send_id: 0,
        },
        Upstream {
            stream: self.stream,
        }))
    }
}

struct Downstream {
    stream: TcpStream,
    send_id: ResponseId,
}

struct Upstream {
    stream: TcpStream,
}

impl Downstream {
    pub fn send_response(&mut self, from: RequestId, resp: &Response) -> IoResult<ResponseId> {
        try!(proto::send_response(&mut self.stream, self.send_id, from, resp));
        let result = Ok(self.send_id);
        self.send_id += 1;
        result
    }
}

impl Upstream {
    pub fn get_request(&mut self) -> IoResult<(RequestId, Request)> {
        proto::recv_request(&mut self.stream)
    }
}
