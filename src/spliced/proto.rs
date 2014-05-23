use std::io::net::tcp::TcpStream;

pub static MAGIC_BYTES: &'static [u8] = bytes!("SPLICEPROTO");
pub static PROTO_VER_MAJOR: i32 = 0;
pub static PROTO_VER_MINOR: i32 = 1;

pub enum NegotiateError {
    ProtocolOutdated,
    ProtocolUnknown,
    ClientDisconnected,
    AuthenticationFailed,
    NegotiateFailed,
}

#[deriving(Clone)]
pub enum AuthType {
    FileSecret(Path), // Unimplemented
}

pub fn negotiate(stream: &mut TcpStream, auth: &Option<AuthType>) -> Result<(), NegotiateError> {
    stream.write(MAGIC_BYTES);
    stream.write_be_i32(PROTO_VER_MAJOR);
    stream.write_be_i32(PROTO_VER_MINOR);
    let magic = match stream.read_exact(MAGIC_BYTES.len()).ok() {
        Some(v) => v,
        None => return Err(NegotiateFailed),
    };
    if magic.as_slice() != MAGIC_BYTES {
        drop(stream);
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
    if major_ver < PROTO_VER_MAJOR {
        drop(stream);
        return Err(ProtocolOutdated)
    }
    match *auth {
        Some(FileSecret(ref p)) => {
            auth_file_secret(p)
        },
        None => Ok(()),
    }
}

// TODO: Implement file based secret authentication method
fn auth_file_secret(path: &Path) -> Result<(), NegotiateError> {
    Err(AuthenticationFailed)
}
