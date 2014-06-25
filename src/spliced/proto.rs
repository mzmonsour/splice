use std::io::net::tcp::TcpStream;
use std::io::{IoResult, IoError, OtherIoError, MemWriter, MemReader};
use serialize::{Encoder, Decoder, Encodable, Decodable};
use serialize::json;

use super::splice;
use splice::proto::raw;
pub use splice::proto::{NegotiateError, ProtocolOutdated, AuthenticationFailed};
pub use splice::proto::StreamHeader;
pub use splice::proto::{RequestHeader, RequestId, Request};
pub use splice::proto::{KeepAlive, OpenBuffer, OpenFile, RunCommand};
pub use splice::proto::{ResponseHeader, ResponseId, Response};
pub use splice::proto::{Ack, Allow, Deny, Handle};
pub use splice::proto::verify_header;

pub static MAGIC_BYTES: &'static [u8] = bytes!("SPLICEPROTO");
pub static PROTO_VER_MAJOR: i32 = 0;
pub static PROTO_VER_MINOR: i32 = 1;

#[deriving(Clone)]
pub enum AuthMethod {
    FileSecret(Path), // Unimplemented
}

pub fn negotiate(stream: &mut TcpStream, auth: &Option<AuthMethod>) -> Result<(), NegotiateError> {
    let StreamHeader {
        magic: magic,
        major_ver: major_ver,
        minor_ver: minor_ver,
    } = try!(verify_header(stream, StreamHeader {
        magic: Vec::from_slice(MAGIC_BYTES),
        major_ver: PROTO_VER_MAJOR,
        minor_ver: PROTO_VER_MINOR,
    }));
    if major_ver < PROTO_VER_MAJOR {
        return Err(ProtocolOutdated)
    }
    match *auth {
        Some(FileSecret(ref p)) => {
            stream.write_be_i32(raw::FileSecret as i32);
            auth_file_secret(stream, p)
        },
        None => {
            stream.write_be_i32(raw::NoAuth as i32);
            Ok(())
        },
    }
}

pub fn recv_request(stream: &mut TcpStream) -> IoResult<(RequestId, Request)> {
    let id = try!(stream.read_be_u64());
    let len = try!(stream.read_be_u64());
    let mut buf = MemReader::new(try!(stream.read_exact(len as uint)));
    let mut decoder = match json::from_reader(&mut buf) {
        Ok(json) => json::Decoder::new(json),
        Err(e) => {
            return Err(IoError {
                kind: OtherIoError,
                desc: "Decode failed",
                detail: None,
            });
        },
    };
    match Decodable::decode(&mut decoder) {
        Ok(v) => Ok((id, v)),
        Err(e) => {
            Err(IoError {
                kind: OtherIoError,
                desc: "Decode failed",
                detail: None,
            })
        },
    }
}

pub fn send_response(stream: &mut TcpStream, id: ResponseId, from: RequestId, data: &Response)
                     -> IoResult<()> {
    let mut buf = MemWriter::new();
    {
        let mut encoder = json::Encoder::new(&mut buf);
        try!(data.encode(&mut encoder));
    }
    let buf = buf.unwrap();
    try!(stream.write_be_u64(id));
    try!(stream.write_be_u64(from));
    try!(stream.write_be_u64(buf.len() as u64));
    try!(stream.write(buf.as_slice()));
    Ok(())
}

// TODO: Implement file based secret authentication method
fn auth_file_secret(stream: &mut TcpStream, path: &Path) -> Result<(), NegotiateError> {
    Err(AuthenticationFailed)
}
