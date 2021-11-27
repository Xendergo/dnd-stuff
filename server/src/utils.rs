use std::future::Future;
use std::net::{IpAddr, UdpSocket};
use std::path::PathBuf;

/// Get the location where the application can store data
pub fn get_data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("You're running an unsupported operating system")
        .join("dnd-stuff")
}

/// Get the server's local ip address that the server can be accessed from
///
/// https://github.com/egmkang/local_ipaddress/blob/master/src/lib.rs
pub fn get_local_ip() -> Option<IpAddr> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    // I don't expect Quad9 to track anything, nor do I expect it to disappear anytime soon, feel free to raise an issue if you disagree
    match socket.connect("9.9.9.9:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip()),
        Err(_) => return None,
    };
}

/// Await the value in an option
///
/// If the value is Some, await what's inside and return the value awaited
/// Otherwise, immediately return None
pub async fn await_option<T>(v: Option<impl Future<Output = T>>) -> Option<T> {
    match v {
        Some(v) => Some(v.await),
        None => None,
    }
}
