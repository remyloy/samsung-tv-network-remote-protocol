# samsung-tv-network-remote-protocol

Implementation of the Samsung TV network remote protocol based
on [sc0ty.pl](https://sc0ty.pl/2012/02/samsung-tv-network-remote-control-protocol/)
and other existing implementations in [Java](https://github.com/sebasgarcep/samsung-tv-remote)
or [NodeJS](https://github.com/Toxblh/samsung-tv-control).

## Example

This example uses this crate with the feature flag `tokio` for the async runtime and the TcpStream
and the crate `local-ip-address` to determine the local IP address.

It connects to the TV, then authenticates the connection and sends a volume up key.

```rs
#[tokio::main]
async fn main() -> Result<()> {
    let tv_ip = "192.168.1.234:55000";
    let local_ip = local_ip_address::local_ip()?.to_string();
    let remote_unique_id = "my-remote";
    let remote_name = "My Remote";

    let stream = TcpStream::connect(tv_ip).await?;
    let rsp = authenticate(
       &mut stream,
       AuthenticateRequest {
           local_ip: &local_ip,
           remote_unique_id,
           remote_name,
       },
    )
    .await?;
    assert!(rsp.result == AuthenticationStatus::AccessGranted);
    send_key(&mut stream, KeyCode::Volup).await?;
    Ok(())
}
```

Access is granted only for the current TCP connection. If either the client or TV disconnects, the authentication process needs to be repeated.
