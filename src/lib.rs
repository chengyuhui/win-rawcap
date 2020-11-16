use anyhow::{bail, Result};
use socket2::{Domain, Socket, Type};
use std::mem::size_of_val;
use std::net::SocketAddr;
use std::net::UdpSocket as StdSocket;
use std::os::windows::io::AsRawSocket;
use std::ptr::null_mut;
use tokio::net::UdpSocket;
use winapi::shared::minwindef::DWORD;
use winapi::um::winsock2;

/// Puts a socket into promiscuous mode so that it can receive all packets.
fn enter_promiscuous(socket: &mut StdSocket) -> Result<()> {
    let rc = unsafe {
        let in_buffer = [winapi::shared::mstcpip::RCVALL_IPLEVEL];
        let mut out: DWORD = 0;
        winsock2::WSAIoctl(
            socket.as_raw_socket() as usize,
            winapi::shared::mstcpip::SIO_RCVALL,
            &in_buffer as *const _ as *mut _,
            size_of_val(&in_buffer) as DWORD,
            null_mut(),
            0,
            &mut out as *mut _,
            null_mut(),
            None,
        )
    };
    if rc == winsock2::SOCKET_ERROR {
        bail!("WSAIoctl() failed: {}", unsafe {
            winsock2::WSAGetLastError()
        });
    } else {
        Ok(())
    }
}

/// Creates a raw socket used to capture packets (disguised as a UdpSocket)
pub fn create_raw_socket() -> Result<UdpSocket> {
    // Create socket
    let socket = Socket::new(Domain::ipv4(), Type::raw(), None)?;
    socket.set_nonblocking(true)?;
    // Bind to lo
    socket.bind(&"127.0.0.1:0".parse::<SocketAddr>().unwrap().into())?;

    let mut socket = socket.into_udp_socket();
    enter_promiscuous(&mut socket)?;

    Ok(UdpSocket::from_std(socket)?)
}