pub mod getifaddrs;
pub mod ifconfig;
pub mod ip;

use std::fmt::Debug;
use std::net::IpAddr;

use nursery_prelude::library_prelude::*;

#[async_trait]
pub trait GetNetInterfaces: Sync {
    async fn get_net_interfaces(&self) -> GetNetInterfacesResult;
}

// TODO: Research the definiton of a network interface
//       Essentially - is it best to represent a net interface as a name and a single address only?
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

    fn new_with_no_address(name: &str) -> Self {
        NetInterface {
            name: name.to_string(),
            ipv4: None,
            ipv6: None,
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

    fn set_ip(&mut self, ip: &IpAddr) {
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
