use std::collections::HashMap;
//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
#[allow(unused_imports)]
use std::sync::Arc;

use zenoh_cfg_properties::Properties;
use zenoh_config::Config;
use zenoh_core::{bail, Result as ZResult};

#[cfg(feature = "transport_quic")]
pub use zenoh_link_quic as quic;
#[cfg(feature = "transport_quic")]
use zenoh_link_quic::{
    LinkManagerUnicastQuic, QuicConfigurator, QuicLocatorInspector, QUIC_LOCATOR_PREFIX,
};
#[cfg(feature = "transport_tcp")]
pub use zenoh_link_tcp as tcp;
#[cfg(feature = "transport_tcp")]
use zenoh_link_tcp::{LinkManagerUnicastTcp, TcpLocatorInspector, TCP_LOCATOR_PREFIX};
#[cfg(feature = "transport_tls")]
pub use zenoh_link_tls as tls;
#[cfg(feature = "transport_tls")]
use zenoh_link_tls::{
    LinkManagerUnicastTls, TlsConfigurator, TlsLocatorInspector, TLS_LOCATOR_PREFIX,
};
#[cfg(feature = "transport_udp")]
pub use zenoh_link_udp as udp;
#[cfg(feature = "transport_udp")]
use zenoh_link_udp::{
    LinkManagerMulticastUdp, LinkManagerUnicastUdp, UdpLocatorInspector, UDP_LOCATOR_PREFIX,
};
#[cfg(all(feature = "transport_unixsock-stream", target_family = "unix"))]
pub use zenoh_link_unixsock_stream as unixsock_stream;
#[cfg(all(feature = "transport_unixsock-stream", target_family = "unix"))]
use zenoh_link_unixsock_stream::{
    LinkManagerUnicastUnixSocketStream, UNIXSOCKSTREAM_LOCATOR_PREFIX,
};

pub use zenoh_link_commons::*;
pub use zenoh_protocol_core::{EndPoint, Locator};

#[derive(Default, Clone)]
pub struct LocatorInspector {
    #[cfg(feature = "transport_quic")]
    quic_inspector: QuicLocatorInspector,
    #[cfg(feature = "transport_tcp")]
    tcp_inspector: TcpLocatorInspector,
    #[cfg(feature = "transport_tls")]
    tls_inspector: TlsLocatorInspector,
    #[cfg(feature = "transport_udp")]
    udp_inspector: UdpLocatorInspector,
}
impl LocatorInspector {
    pub async fn is_multicast(&self, locator: &Locator) -> ZResult<bool> {
        #[allow(unused_imports)]
        use zenoh_link_commons::LocatorInspector;
        let protocol = locator.protocol();
        match protocol {
            #[cfg(feature = "transport_tcp")]
            TCP_LOCATOR_PREFIX => self.tcp_inspector.is_multicast(locator).await,
            #[cfg(feature = "transport_udp")]
            UDP_LOCATOR_PREFIX => self.udp_inspector.is_multicast(locator).await,
            #[cfg(feature = "transport_tls")]
            TLS_LOCATOR_PREFIX => self.tls_inspector.is_multicast(locator).await,
            #[cfg(feature = "transport_quic")]
            QUIC_LOCATOR_PREFIX => self.quic_inspector.is_multicast(locator).await,
            #[cfg(all(feature = "transport_unixsock-stream", target_family = "unix"))]
            UNIXSOCKSTREAM_LOCATOR_PREFIX => Ok(false),
            _ => bail!("Unsupported protocol: {}.", protocol),
        }
    }
}
#[derive(Default)]
pub struct LinkConfigurator {
    #[cfg(feature = "transport_quic")]
    quic_inspector: QuicConfigurator,
    #[cfg(feature = "transport_tls")]
    tls_inspector: TlsConfigurator,
}
impl LinkConfigurator {
    #[allow(unused_variables, unused_mut)]
    pub async fn configurations(
        &self,
        config: &Config,
    ) -> (
        HashMap<String, Properties>,
        HashMap<String, zenoh_core::Error>,
    ) {
        let mut configs = HashMap::new();
        let mut errors = HashMap::new();
        let mut insert_config = |proto: String, cfg: ZResult<Properties>| match cfg {
            Ok(v) => {
                configs.insert(proto, v);
            }
            Err(e) => {
                errors.insert(proto, e);
            }
        };
        #[cfg(feature = "transport_quic")]
        {
            insert_config(
                QUIC_LOCATOR_PREFIX.into(),
                self.quic_inspector.inspect_config(config).await,
            );
        }
        #[cfg(feature = "transport_tls")]
        {
            insert_config(
                TLS_LOCATOR_PREFIX.into(),
                self.tls_inspector.inspect_config(config).await,
            );
        }
        (configs, errors)
    }
}

/*************************************/
/*             UNICAST               */
/*************************************/

pub struct LinkManagerBuilderUnicast;

impl LinkManagerBuilderUnicast {
    pub fn make(_manager: NewLinkChannelSender, protocol: &str) -> ZResult<LinkManagerUnicast> {
        match protocol {
            #[cfg(feature = "transport_tcp")]
            TCP_LOCATOR_PREFIX => Ok(Arc::new(LinkManagerUnicastTcp::new(_manager))),
            #[cfg(feature = "transport_udp")]
            UDP_LOCATOR_PREFIX => Ok(Arc::new(LinkManagerUnicastUdp::new(_manager))),
            #[cfg(feature = "transport_tls")]
            TLS_LOCATOR_PREFIX => Ok(Arc::new(LinkManagerUnicastTls::new(_manager))),
            #[cfg(feature = "transport_quic")]
            QUIC_LOCATOR_PREFIX => Ok(Arc::new(LinkManagerUnicastQuic::new(_manager))),
            #[cfg(all(feature = "transport_unixsock-stream", target_family = "unix"))]
            UNIXSOCKSTREAM_LOCATOR_PREFIX => {
                Ok(Arc::new(LinkManagerUnicastUnixSocketStream::new(_manager)))
            }
            _ => bail!("Unicast not supported for {} protocol", protocol),
        }
    }
}

/*************************************/
/*            MULTICAST              */
/*************************************/

pub struct LinkManagerBuilderMulticast;

impl LinkManagerBuilderMulticast {
    pub fn make(protocol: &str) -> ZResult<LinkManagerMulticast> {
        match protocol {
            #[cfg(feature = "transport_udp")]
            UDP_LOCATOR_PREFIX => Ok(Arc::new(LinkManagerMulticastUdp::default())),
            _ => bail!("Multicast not supported for {} protocol", protocol),
        }
    }
}

pub const WBUF_SIZE: usize = 64;
