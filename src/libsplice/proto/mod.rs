use std::io::net::tcp::TcpStream;
use std::io::{IoResult, IoError, OtherIoError, MemWriter, MemReader};
use serialize::{Encoder, Decoder, Encodable, Decodable};
use serialize::json;
use Object;

pub mod raw;

pub static MAGIC_BYTES: &'static [u8] = b"SPLICEPROTO";
pub static PROTO_VER_MAJOR: i32 = 0;
pub static PROTO_VER_MINOR: i32 = 1;

pub enum NegotiateError {
    ProtocolOutdated,
    ProtocolUnknown,
    RemoteDisconnected,
    AuthenticationFailed,
    NegotiateFailed,
}

pub struct StreamHeader {
    pub magic: Vec<u8>,
    pub major_ver: i32,
    pub minor_ver: i32,
}

pub type RequestId = u64;
pub struct RequestHeader {
    pub id: RequestId,
    pub len: u64,
}

#[deriving(Encodable, Decodable)]
pub enum Request {
    KeepAlive,
    OpenBuffer(Object),
    OpenFile(Path),
    Close(Object),
    RunCommand, // Implement me
}

pub type ResponseId = u64;
pub struct ResponseHeader {
    pub id: ResponseId,
    pub from: RequestId,
    pub len: u64,
}

#[deriving(Encodable, Decodable)]
pub enum Response {
    Ack,
    Allow,
    Deny,
    Handle(Object),
}

pub fn negotiate(stream: &mut TcpStream) -> Result<(), NegotiateError> {
    let StreamHeader {
        major_ver: major_ver, ..
    } = try!(verify_header(stream, StreamHeader {
        magic: Vec::from_slice(MAGIC_BYTES),
        major_ver: PROTO_VER_MAJOR,
        minor_ver: PROTO_VER_MINOR,
    }));
    if major_ver < PROTO_VER_MAJOR {
        return Err(ProtocolOutdated)
    }
    let auth = match stream.read_be_i32().ok() {
        Some(v) => v,
        None => return Err(NegotiateFailed),
    };
    match ::std::num::from_int::<raw::AuthType>(auth as int) {
        Some(raw::NoAuth) => Ok(()),
        Some(raw::FileSecret) => Err(AuthenticationFailed),
        None => Err(AuthenticationFailed),
    }
}

pub fn send_request(stream: &mut TcpStream, id: RequestId, data: &Request) -> IoResult<()> {
    let mut buf = MemWriter::new();
    {
        let mut encoder = json::Encoder::new(&mut buf);
        try!(data.encode(&mut encoder));
    }
    let buf = buf.unwrap();
    try!(stream.write_be_u64(id));
    try!(stream.write_be_u64(buf.len() as u64));
    try!(stream.write(buf.as_slice()));
    Ok(())
}

pub fn recv_response(stream: &mut TcpStream) -> IoResult<(ResponseHeader, Response)> {
    let header = ResponseHeader {
        id: try!(stream.read_be_u64()),
        from: try!(stream.read_be_u64()),
        len: try!(stream.read_be_u64()),
    };
    let mut buf = MemReader::new(try!(stream.read_exact(header.len as uint)));
    let mut decoder = match json::from_reader(&mut buf) {
        Ok(json) => json::Decoder::new(json),
        Err(..) => {
            return Err(IoError {
                kind: OtherIoError,
                desc: "Decode failed",
                detail: None,
            });
        }
    };
    match Decodable::decode(&mut decoder) {
        Ok(v) => Ok((header, v)),
        Err(..) => {
            Err(IoError {
                kind: OtherIoError,
                desc: "Decode failed",
                detail: None,
            })
        },
    }
}

pub fn verify_header(stream: &mut TcpStream, header: StreamHeader)
                     -> Result<StreamHeader, NegotiateError> {
    match stream.write(header.magic.as_slice()) {
        Ok(..) => (),
        Err(..) => return Err(NegotiateFailed),
    };
    match stream.write_be_i32(header.major_ver) {
        Ok(..) => (),
        Err(..) => return Err(NegotiateFailed),
    };
    match stream.write_be_i32(header.minor_ver) {
        Ok(..) => (),
        Err(..) => return Err(NegotiateFailed),
    };
    let magic = match stream.read_exact(MAGIC_BYTES.len()).ok() {
        Some(v) => v,
        None => return Err(NegotiateFailed),
    };
    if magic.as_slice() != MAGIC_BYTES {
        return Err(ProtocolUnknown)
    }
    let major_ver = match stream.read_be_i32().ok() {
        Some(v) => v,
        None => return Err(NegotiateFailed),
    };
    let minor_ver = match stream.read_be_i32().ok() {
        Some(v) => v,
        None => return Err(NegotiateFailed),
    };
    Ok(StreamHeader {
        magic: magic,
        major_ver: major_ver,
        minor_ver: minor_ver,
    })
}
