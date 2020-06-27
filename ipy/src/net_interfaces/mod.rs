pub mod ifconfig;
pub mod ip;

use anyhow::Result;
use async_trait::async_trait;
use regex::{Captures, Match, Regex};
use std::fmt::Debug;
use std::net::IpAddr;
use std::process::Output;
use thiserror::Error;
use toolbox_rustbase::CLIProgram;

#[async_trait]
pub trait GetNetInterfaces: NetCLIProgram {
    async fn get_net_interfaces(&self) -> GetNetInterfacesResult {
        self.parse_output(self.call().await?).await
    }
}

// TODO: Network interfaces with multiple v4 or v6 addresses will just give the first for now
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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
}

#[derive(Error, Debug)]
pub enum GetNetInterfacesError {
    #[error("No name found for network interface. Output:\n{0}")]
    NoInterfaceNameFound(String),
}

const REGEX_GROUP_NAME: &str = "interface_name";
const REGEX_GROUP_IPV4: &str = "interface_ip_v4";
const REGEX_GROUP_IPV6: &str = "interface_ip_v6";

pub type GetNetInterfacesResult = Result<Vec<NetInterface>>;

#[async_trait]
pub trait NetCLIProgram: CLIProgram<GetNetInterfacesResult> {
    fn get_regex(&self) -> &Regex;

    async fn parse_output_to_net_interfaces(&self, output: Output) -> GetNetInterfacesResult {
        let mut net_interfaces = vec![];
        let debug_output = format!("{:?}", output);

        let s = String::from_utf8(output.stdout)?;
        for c in self.get_regex().captures_iter(&s) {
            let interface_name = get_interface_name(&c, &debug_output)?;
            let (ipv4, ipv6) = get_ip_addresses(&c)?;

            net_interfaces.push(NetInterface::new(&interface_name.to_string(), &ipv4, &ipv6));
        }

        Ok(net_interfaces)
    }
}

fn get_interface_name(c: &Captures, debug_output: &str) -> Result<String> {
    Ok(c.name(REGEX_GROUP_NAME)
        .ok_or_else(|| GetNetInterfacesError::NoInterfaceNameFound((*debug_output).to_string()))?
        .as_str()
        .into())
}

fn get_ip_addresses(c: &Captures) -> Result<(Option<IpAddr>, Option<IpAddr>)> {
    let parse_ip_addr = |m: Match| m.as_str().parse::<IpAddr>().ok()?.into();

    Ok(match (c.name(REGEX_GROUP_IPV4), c.name(REGEX_GROUP_IPV6)) {
        (None, None) => (None, None),
        (Some(ipv4_match), None) => (parse_ip_addr(ipv4_match), None),
        (None, Some(ipv6_match)) => (None, parse_ip_addr(ipv6_match)),
        (Some(ipv4_match), Some(ipv6_match)) => {
            (parse_ip_addr(ipv4_match), parse_ip_addr(ipv6_match))
        }
    })
}
