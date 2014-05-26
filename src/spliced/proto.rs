use std::io::net::tcp::TcpStream;

use super::splice;
pub use splice::proto::{NegotiateError, ProtocolOutdated, AuthenticationFailed};
pub use splice::proto::StreamHeader;
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
            stream.write_be_i32(splice::proto::FileSecret as i32);
            auth_file_secret(stream, p)
        },
        None => {
            stream.write_be_i32(splice::proto::NoAuth as i32);
            Ok(())
        },
    }
}

// TODO: Implement file based secret authentication method
fn auth_file_secret(stream: &mut TcpStream, path: &Path) -> Result<(), NegotiateError> {
    Err(AuthenticationFailed)
}
