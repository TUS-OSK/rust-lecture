use std::{fmt, collections::BTreeMap, net::{IpAddr, Ipv4Addr}};

use smoltcp::{iface::{NeighborCache, Routes, InterfaceBuilder}, wire::{IpAddress, EthernetAddress, IpCidr, Ipv4Address}, socket::{TcpSocket, TcpSocketBuffer}, phy::TunTapInterface};
use url::Url;

#[derive(Debug)]
enum HttpState {
    Connect,
    Request,
    Response,
}

#[derive(Debug)]
pub enum UpstreamError {
    Network(smoltcp::Error),
    InvalidUrl,
    Content(std::str::Utf8Error)
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<smoltcp::Error> for UpstreamError {
    fn from(error: smoltcp::Error) -> Self {
        UpstreamError::Network(error)
    }
}

//65565
fn random_port() -> u16 {
    49152 + rand::random::<u16>() % 16384
}

pub fn get(
    tap: TunTapInterface,
    mac: EthernetAddress,
    addr: IpAddr,
    url: Url,
) -> Result<(), UpstreamError> {
    let domain_name = url.host_str().ok_or(UpstreamError::InvalidUrl)?;

    let neighbor_cache = NeighborCache::new(BTreeMap::new());

    let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);

    let ip_addrs = [IpCidr::new(IpAddress::v4(192, 168, 42, 1), 24)];
    
    let fd = tap.as_raw_fd();
    let mut routes = Routes::new(BTreeMap::new());
    let default_gateway = Ipv4Address::new(192, 168, 42, 100);
    routes.add_default_ipv4_route(default_gateway).unwrap();
    let mut iface = InterfaceBuilder::new(tap, vec![])
        .hardware_addr(smoltcp::wire::HardwareAddress::Ethernet(mac))
        .neighbor_cache(neighbor_cache)
        .ip_addrs(ip_addrs)
        .routes(routes)
        .finalize();

    let mut sockets = SocketSet::new(vec![]);
}