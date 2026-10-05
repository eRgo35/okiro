//! Default I/O implementations used by the `okiro` binary.
//!
//! These are deliberately tiny wrappers around `socket2` and
//! `std::process::Command`. Tests use the trait fakes in
//! `okiro_core::io` instead.

use std::net::{Ipv4Addr, SocketAddrV4};

use okiro_core::host::MacAddr;
use okiro_core::io::{WolError, WolSender};

/// Default `WolSender` backed by a `socket2` UDP socket with
/// `SO_BROADCAST` enabled. Sends to `255.255.255.255:9`, the canonical
/// WOL port.
pub struct UdpWolSender;

impl WolSender for UdpWolSender {
    fn send(&self, mac: MacAddr) -> Result<(), WolError> {
        let packet = build_magic_packet(mac.as_bytes());

        let socket = socket2::Socket::new(
            socket2::Domain::IPV4,
            socket2::Type::DGRAM,
            Some(socket2::Protocol::UDP),
        )
        .map_err(|e| WolError::Send(format!("socket new: {e}")))?;

        socket
            .set_broadcast(true)
            .map_err(|e| WolError::Send(format!("set_broadcast: {e}")))?;

        let target = SocketAddrV4::new(Ipv4Addr::BROADCAST, 9);
        socket
            .send_to(&packet, &socket2::SockAddr::from(target))
            .map_err(|e| WolError::Send(format!("send_to: {e}")))?;

        Ok(())
    }
}

/// Construct a 102-byte WOL magic packet: 6 × 0xFF followed by the
/// 6-byte MAC repeated 16 times.
#[must_use]
pub fn build_magic_packet(mac: [u8; 6]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(6 + 16 * 6);
    packet.extend_from_slice(&[0xFF; 6]);
    for _ in 0..16 {
        packet.extend_from_slice(&mac);
    }
    packet
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;

    #[test]
    fn magic_packet_layout() {
        let mac = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let pkt = build_magic_packet(mac);
        assert_eq!(pkt.len(), 102);
        assert!(pkt[..6].iter().all(|b| *b == 0xFF));
        for i in 0..16 {
            assert_eq!(&pkt[6 + i * 6..6 + (i + 1) * 6], &mac);
        }
    }
}
