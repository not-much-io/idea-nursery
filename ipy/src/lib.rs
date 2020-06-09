mod ifconfig;
mod ip;

use anyhow::Result;
use async_trait::async_trait;
use lazy_static::lazy_static;
use std::net::IpAddr;
use std::net::{Ipv4Addr, Ipv6Addr};

lazy_static! {
    static ref IPV4_LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    static ref IPV6_LOCALHOST: IpAddr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
}

#[async_trait]
trait GetPublicIP {
    async fn get_public_ip(&self) -> Result<IpAddr>;
}

#[async_trait]
trait GetPrivateIP {
    async fn get_private_ip(&self) -> Result<Vec<IpAddr>>;
}
