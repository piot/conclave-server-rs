/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/conclave-server-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use std::net::UdpSocket;

use log::*;

fn main() -> std::io::Result<()> {
    {
        stderrlog::new()
            .module(module_path!())
            .verbosity(4)
            .init()
            .unwrap();
        info!("started!");
        let udp_socket = UdpSocket::bind("127.0.0.1:27004")?;

        let mut buf = [0; 1200];

        let (octet_count, src) = udp_socket.recv_from(&mut buf)?;
        debug!("received octet count {}", octet_count);

        let buf = &mut buf[..octet_count];

        udp_socket.send_to(buf, &src)?;
    }
    Ok(())
}