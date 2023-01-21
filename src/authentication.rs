use crate::protocol::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Authenticates this client with the TV as a network remote.
///
/// This struct is used by the [`create_authenticate_msg`] function of this module to
/// create the message, which needs to be sent to the TV.
#[derive(Debug)]
pub struct AuthenticateRequest<'a> {
    /// The IP address of the client.
    pub local_ip: &'a str,
    /// A value to distinguish network remotes, e.g. the MAC address of the client.
    pub remote_unique_id: &'a str,
    /// The name of the network remote, as it will be displayed on the confirmation screen and in the menu of the TV.
    pub remote_name: &'a str,
}

/// The parsed contents of the response message to an authentication request.
///
/// This struct is returned by the [`parse_authenticate_rsp`] function of this module after
/// the message was successfully parsed.
#[derive(Debug)]
pub struct AuthenticateResponse {
    _reserved: u8,
    /// The device name of the sender, e.g. iapp.samsung or unknown.livingroom.samsung.
    /// Mostly included for debugging purposes.
    pub device_name: String,
    /// The result of the authentication attempt.
    pub result: AuthenticationResult,
}

/// The possible outcomes of an authentication attempt.
#[derive(Debug, PartialEq, Eq)]
pub enum AuthenticationResult {
    /// The authentication attempt was confirmed by the user, or the client was already known.
    AccessGranted,
    /// The authentication attempt was denied by the user.
    AccessDenied,
    /// Waiting for user to grant or deny access for your app.
    WaitingForUserConfirmation,
    /// Timeout or cancelled by user.
    TimeoutOrCancelledByUser,
}

/// Create an authentication request.
pub fn create_authenticate_msg(
    AuthenticateRequest {
        local_ip,
        remote_unique_id,
        remote_name,
    }: AuthenticateRequest,
) -> Vec<u8> {
    let mut payload: Vec<u8> = vec![0x64, 0x00];
    extend_with_encoded_string(&mut payload, local_ip);
    extend_with_encoded_string(&mut payload, remote_unique_id);
    extend_with_encoded_string(&mut payload, remote_name);
    create_request(&payload)
}

/// Parses the authentication response.
pub fn parse_authenticate_rsp(buf: &[u8]) -> Result<AuthenticateResponse> {
    const PAYLOAD_OFFSET: usize = 0x11;
    let prelude: [u8; PAYLOAD_OFFSET] = buf[0..PAYLOAD_OFFSET].try_into()?;
    let reserved = prelude[0];
    assert!(reserved == 0 || reserved == 2);
    let device_name = parse_string(&prelude[1..])?;
    let result = parse_authentication_result(&buf[PAYLOAD_OFFSET..])?;
    Ok(AuthenticateResponse {
        _reserved: reserved,
        device_name: device_name,
        result,
    })
}

fn parse_authentication_result(buf: &[u8]) -> Result<AuthenticationResult> {
    let kind = buf.get(0).ok_or(invalid_data(
        "payload size too small for authentication result".to_string(),
    ))?;
    match kind {
        0x64 => {
            let prelude: [u8; 4] = buf[0..4].try_into()?;
            match prelude {
                [0x64, 0, 1, 0] => Ok(AuthenticationResult::AccessGranted),
                [0x64, 0, 0, 0] => Ok(AuthenticationResult::AccessDenied),
                _ => {
                    Err(invalid_data(format!("unexpected payload for access {:?}", prelude)).into())
                }
            }
        }
        0x0A => {
            let prelude: [u8; 6] = buf[0..6].try_into()?;
            match prelude {
                [0x0A, 0, 2, 0, 0, 0] => Ok(AuthenticationResult::WaitingForUserConfirmation),
                _ => Err(invalid_data(format!(
                    "unexpected payload for waiting for user confirmation {:?}",
                    prelude
                ))
                .into()),
            }
        }
        0x65 => {
            let prelude: [u8; 2] = buf[0..2].try_into()?;
            match prelude {
                [0x65, 0] => Ok(AuthenticationResult::TimeoutOrCancelledByUser),
                _ => Err(invalid_data(format!(
                    "unexpected payload for timeout or cancelled by user {:?}",
                    prelude
                ))
                .into()),
            }
        }
        _ => Err(invalid_data(format!(
            "unexpected discriminator for authentication_result {}",
            kind
        ))
        .into()),
    }
}
