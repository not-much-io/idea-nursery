pub mod ifconfig;
pub mod ip;

use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;
use std::net::IpAddr;
use thiserror::Error;
use toolbox_rustbase::CLIProgram;

#[async_trait]
pub trait GetNetInterfaces: CLIProgram<GetNetInterfacesResult> + Sync {
    async fn get_net_interfaces(&self) -> GetNetInterfacesResult {
        self.parse_output(self.call().await?).await
    }
}

// TODO: Network interfaces with multiple v4 or v6 addresses will just give the first for now
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct NetInterface {
    pub name: String,
    pub ipv4: Option<IpAddr>,
    pub ipv6: Option<IpAddr>,
}

impl NetInterface {
    fn new(name: &str, ipv4: &Option<IpAddr>, ipv6: &Option<IpAddr>) -> Self {
        NetInterface {
            name: name.to_string(),
            ipv4: *ipv4,
            ipv6: *ipv6,
        }
    }

    fn new_with_single_ip(name: &str, ip: &IpAddr) -> Self {
        match ip {
            IpAddr::V4(_) => NetInterface {
                name: name.to_string(),
                ipv4: Some(*ip),
                ipv6: None,
            },
            IpAddr::V6(_) => NetInterface {
                name: name.to_string(),
                ipv4: None,
                ipv6: Some(*ip),
            },
        }
    }

    fn set_ip(self: &mut Self, ip: &IpAddr) {
        match ip {
            IpAddr::V4(_) => {
                self.ipv4 = Some(*ip);
            }
            IpAddr::V6(_) => {
                self.ipv6 = Some(*ip);
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum GetNetInterfacesError {
    #[error("No name found for network interface.")]
    NoNameForInterfaceFound(),
    #[error("No ip found for network interface.")]
    NoAddrForInterfaceFound(),
}

pub type GetNetInterfacesResult = Result<Vec<NetInterface>>;
