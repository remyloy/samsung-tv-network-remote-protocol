use crate::authentication::*;
use crate::protocol::*;
use crate::send_key::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Executes the authentication hand-shake with the TV.
pub async fn authenticate<'a>(
    stream: &mut TcpStream,
    authenticate_request: AuthenticateRequest<'a>,
) -> Result<AuthenticateResponse> {
    let data = create_authenticate_msg(authenticate_request);
    stream.write(&data).await?;
    let mut buf = [0u8; 1024];
    let read_bytes = stream.read(&mut buf).await?;
    let rsp = parse_authenticate_rsp(&buf[0..read_bytes])?;
    Ok(rsp)
}

/// Sends the key to the TV.
pub async fn send_key(stream: &mut TcpStream, key_code: KeyCode) -> Result<SendKeyResponse> {
    let data = create_send_key_msg(SendKeyRequest { key_code: key_code })?;
    let written_bytes = stream.write(&data).await?;
    if written_bytes == 0 {
        return Err(not_connected("connection closed").into());
    }

    let mut buf = [0u8; 1024];
    let read_bytes = stream.read(&mut buf).await?;
    if read_bytes == 0 {
        return Err(not_connected("connection closed").into());
    }

    let rsp = parse_send_key_rsp(&buf[0..read_bytes])?;
    Ok(rsp)
}
