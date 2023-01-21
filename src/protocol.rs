type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) fn create_request(payload: &[u8]) -> Vec<u8> {
    let mut data = vec![0u8];
    extend_with_raw_string(&mut data, NAME);

    let payload_len = payload.len() as u16;
    data.extend_from_slice(&payload_len.to_le_bytes());
    data.extend_from_slice(&payload);

    data
}

static NAME: &str = "iphone.iapp.samsung";

fn extend_with_raw_string(buf: &mut Vec<u8>, s: &str) {
    let str_len = s.len() as u16;
    buf.extend_from_slice(&str_len.to_le_bytes());
    buf.extend_from_slice(&s.as_bytes());
}

pub(crate) fn extend_with_encoded_string(buf: &mut Vec<u8>, s: &str) {
    use base64::prelude::*;
    let encoded_str = BASE64_STANDARD.encode(s);
    let encoded_str_len = encoded_str.len() as u16;
    buf.extend_from_slice(&encoded_str_len.to_le_bytes());
    buf.extend_from_slice(&encoded_str.as_bytes());
}

pub(crate) fn parse_string(buf: &[u8]) -> Result<String> {
    let len = u16::from_le_bytes(buf[0..2].try_into()?);
    let str_buf = &buf[2..2 + len as usize];
    let str = String::from_utf8_lossy(str_buf).to_string();
    Ok(str)
}

pub(crate) fn decode_string(buf: &[u8]) -> Result<String> {
    use base64::prelude::*;
    let len = u16::from_le_bytes(buf[0..2].try_into()?);
    println!("decode_string len {}", len);
    let str_buf = &buf[2..2 + len as usize];
    let encoded_str = String::from_utf8_lossy(str_buf).to_string();
    let decoded_str = BASE64_STANDARD.decode(encoded_str)?;
    Ok(String::from_utf8_lossy(&decoded_str).to_string())
}

pub(crate) fn invalid_data(msg: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, msg)
}

pub(crate) fn not_connected(msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::NotConnected, msg)
}
