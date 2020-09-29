use nix::{ifaddrs, sys::socket::SockAddr};

use crate::net_interfaces::{GetNetInterfaces, GetNetInterfacesResult, NetInterface};

use nursery_prelude::library_prelude::*;

// https://man7.org/linux/man-pages/man3/getifaddrs.3.html
pub struct GetIfAddrs();

impl GetIfAddrs {
    pub fn new() -> GetIfAddrs {
        GetIfAddrs {}
    }
}

impl Default for GetIfAddrs {
    fn default() -> Self {
        GetIfAddrs::new()
    }
}

#[async_trait]
impl GetNetInterfaces for GetIfAddrs {
    async fn get_net_interfaces(&self) -> GetNetInterfacesResult {
        let mut net_interfaces = vec![];
        let iaddr_iter = ifaddrs::getifaddrs()?;

        for iaddr in iaddr_iter {
            let name = iaddr.interface_name;
            let _ = match iaddr.address {
                None => None,
                Some(address) => match address {
                    SockAddr::Inet(inet_addr) => Some(inet_addr.ip()),
                    _ => None,
                },
            };

            let net_interface = NetInterface::new_with_no_address(&name);
            net_interfaces.push(net_interface);
        }

        Ok(net_interfaces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_actual_call() {
        let xs = GetIfAddrs::default().get_net_interfaces().await.unwrap();
        for x in xs {
            dbg!(x);
        }
    }
}
