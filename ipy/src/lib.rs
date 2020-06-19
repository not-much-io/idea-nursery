mod ifconfig;
mod ip;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use regex::Regex;
use std::net::IpAddr;
use std::process::Output;
use toolbox_rustbase::CLIProgram;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct NetworkInterface {
    name:       String,
    ip_addr_v4: Option<IpAddr>,
    ip_addr_v6: Option<IpAddr>,
}

impl NetworkInterface {
    fn new(name: &String, ip_addr_v4: &Option<IpAddr>, ip_addr_v6: &Option<IpAddr>) -> Self {
        NetworkInterface {
            name:       name.clone(),
            ip_addr_v4: ip_addr_v4.clone(),
            ip_addr_v6: ip_addr_v6.clone(),
        }
    }
}

type NetProgramResult = Result<Vec<NetworkInterface>>;

const REGEX_GROUP_NAME: &str = "interface_name";
const REGEX_GROUP_IP_V4: &str = "interface_ip_v4";
const REGEX_GROUP_IP_V6: &str = "interface_ip_v6";

#[async_trait]
trait NetCLIProgram: CLIProgram<NetProgramResult> {
    fn get_regex(&self) -> &Regex;

    async fn parse_output_to_net_interfaces(&self, output: Output) -> NetProgramResult {
        let mut net_interfaces = vec![];

        let s = String::from_utf8(output.stdout)?;
        for c in self.get_regex().captures_iter(&s) {
            let name = c
                .name(REGEX_GROUP_NAME)
                .ok_or(anyhow!("Missing name regex group"))?
                .as_str();

            let (ip_v4, ip_v6) = match (c.name(REGEX_GROUP_IP_V4), c.name(REGEX_GROUP_IP_V6)) {
                (None, None) => {
                    return Err(anyhow!(
                        "Unable to find neither a V4 or V6 ip address for interface {}",
                        name,
                    ))
                }
                (Some(ip_v4), Some(ip_v6)) => (
                    ip_v4.as_str().parse::<IpAddr>()?.into(),
                    ip_v6.as_str().parse::<IpAddr>()?.into(),
                ),
                (Some(ip_v4), None) => (ip_v4.as_str().parse::<IpAddr>()?.into(), None),
                (None, Some(ip_v6)) => (None, ip_v6.as_str().parse::<IpAddr>()?.into()),
            };

            net_interfaces.push(NetworkInterface::new(&name.to_string(), &ip_v4, &ip_v6));
        }
        Ok(net_interfaces)
    }
}

#[async_trait]
trait GetPublicInterfaces: NetCLIProgram {
    async fn get_public_interfaces(&self) -> NetProgramResult {
        self.parse_output(self.call().await?).await
    }
}

#[async_trait]
trait GetPrivateInterfaces: NetCLIProgram {
    async fn get_private_interfaces(&self) -> NetProgramResult {
        self.parse_output(self.call().await?).await
    }
}
