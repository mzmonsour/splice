extern crate splice;

use std::io::net::tcp::{TcpListener, TcpAcceptor, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::IoResult;

use super::proto;

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
        },
        Upstream {
            stream: self.stream,
        }))
    }
}

struct Downstream {
    stream: TcpStream,
}

struct Upstream {
    stream: TcpStream,
}
