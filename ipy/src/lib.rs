mod ifconfig;
mod ip;

use anyhow::Result;
use async_trait::async_trait;
use regex::{Match, Regex};
use std::net::IpAddr;
use std::process::Output;
use thiserror::Error;
use toolbox_rustbase::CLIProgram;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct NetworkInterface {
    name:       String,
    ip_addr_v4: Option<IpAddr>,
    ip_addr_v6: Option<IpAddr>,
}

impl NetworkInterface {
    fn new(name: &str, ip_addr_v4: &Option<IpAddr>, ip_addr_v6: &Option<IpAddr>) -> Self {
        NetworkInterface {
            name:       name.to_string(),
            ip_addr_v4: *ip_addr_v4,
            ip_addr_v6: *ip_addr_v6,
        }
    }
}

type GetNetInterfacesResult = Result<Vec<NetworkInterface>>;

const REGEX_GROUP_NAME: &str = "interface_name";
const REGEX_GROUP_IP_V4: &str = "interface_ip_v4";
const REGEX_GROUP_IP_V6: &str = "interface_ip_v6";

#[derive(Error, Debug)]
enum GetNetInterfacesError {
    #[error("No name found for network interface")]
    NoInterfaceNameFound,
    #[error("No IPv4 or IPv6 found for network interface {0}")]
    NoIpAddressFound(String),
}

#[async_trait]
trait NetCLIProgram: CLIProgram<GetNetInterfacesResult> {
    fn get_regex(&self) -> &Regex;

    async fn parse_output_to_net_interfaces(&self, output: Output) -> GetNetInterfacesResult {
        let mut net_interfaces = vec![];

        let s = String::from_utf8(output.stdout)?;
        for c in self.get_regex().captures_iter(&s) {
            let name = c
                .name(REGEX_GROUP_NAME)
                .ok_or(GetNetInterfacesError::NoInterfaceNameFound)?
                .as_str();

            let parse_ip_addr =
                |m: Match| -> Option<IpAddr> { m.as_str().parse::<IpAddr>().ok()?.into() };

            let (ip_addr_v4, ip_addr_v6) =
                match (c.name(REGEX_GROUP_IP_V4), c.name(REGEX_GROUP_IP_V6)) {
                    (None, None) => {
                        return Err(GetNetInterfacesError::NoIpAddressFound(name.to_string()).into())
                    }
                    (Some(ip_v4_match), Some(ip_v6_match)) => {
                        (parse_ip_addr(ip_v4_match), parse_ip_addr(ip_v6_match))
                    }
                    (Some(ip_v4_match), None) => (parse_ip_addr(ip_v4_match), None),
                    (None, Some(ip_v6_match)) => (None, parse_ip_addr(ip_v6_match)),
                };

            net_interfaces.push(NetworkInterface::new(
                &name.to_string(),
                &ip_addr_v4,
                &ip_addr_v6,
            ));
        }

        Ok(net_interfaces)
    }
}

#[async_trait]
trait GetPublicInterfaces: NetCLIProgram {
    async fn get_public_interfaces(&self) -> GetNetInterfacesResult {
        self.parse_output(self.call().await?).await
    }
}

#[async_trait]
trait GetPrivateInterfaces: NetCLIProgram {
    async fn get_private_interfaces(&self) -> GetNetInterfacesResult {
        self.parse_output(self.call().await?).await
    }
}
