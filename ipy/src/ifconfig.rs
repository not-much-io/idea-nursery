use crate::{GetPrivateIP, IPV4_LOCALHOST, IPV6_LOCALHOST};
use anyhow::Result;
use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use std::net::IpAddr;
use std::process::{Command, Output};
use toolbox_rustbase::CLIProgram;

// https://man7.org/linux/man-pages/man8/ifconfig.8.html
struct IfConfig();

lazy_static! {
    // Capture the ip definition from ip-s output.
    // Contains three groups:
    //  (inet6|inet)    The network interface definition beginning
    //  (.*?)           The actual IP - this is the interesting group
    //  /s              First space character
    static ref RE: Regex = Regex::new(r#"(inet6|inet) (.*?)\s"#).unwrap();
}

#[async_trait]
impl CLIProgram<Result<Vec<IpAddr>>> for IfConfig {
    fn name(&self) -> &str {
        "ifconfig"
    }

    async fn call(&self) -> Result<Output> {
        Ok(Command::new(self.name()).arg("-a").output()?)
    }

    async fn parse_output(&self, output: Output) -> Result<Vec<IpAddr>> {
        let s = String::from_utf8(output.stdout)?;
        Ok(RE
            .captures_iter(&s)
            .filter_map(|c| c.get(2))
            .map(|m| m.as_str().parse::<IpAddr>())
            .filter_map(Result::ok)
            .filter(|ip| *ip != *IPV4_LOCALHOST && *ip != *IPV6_LOCALHOST)
            .collect())
    }
}

#[async_trait]
impl GetPrivateIP for IfConfig {
    async fn get_private_ip(&self) -> Result<Vec<IpAddr>> {
        self.parse_output(self.call().await?).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;
    use tokio;

    const IP_OUTPUT: &str = "
br-b83013461f0c: flags=4099<UP,BROADCAST,MULTICAST>  mtu 1500
    inet 172.23.0.1  netmask 255.255.0.0  broadcast 172.23.255.255
    ether 02:42:5d:8c:83:bc  txqueuelen 0  (Ethernet)
    RX packets 0  bytes 0 (0.0 B)
    RX errors 0  dropped 0  overruns 0  frame 0
    TX packets 0  bytes 0 (0.0 B)
    TX errors 0  dropped 0 overruns 0  carrier 0  collisions 0

docker0: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500
    inet 172.17.0.1  netmask 255.255.0.0  broadcast 172.17.255.255
    inet6 fe80::42:79ff:fe2b:f5c3  prefixlen 64  scopeid 0x20<link>
    ether 02:42:79:2b:f5:c3  txqueuelen 0  (Ethernet)
    RX packets 199772  bytes 10417009 (9.9 MiB)
    RX errors 0  dropped 0  overruns 0  frame 0
    TX packets 376100  bytes 558495778 (532.6 MiB)
    TX errors 0  dropped 0 overruns 0  carrier 0  collisions 0

enp34s0: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500
    inet 192.168.0.11  netmask 255.255.255.0  broadcast 192.168.0.255
    inet6 fe80::6954:9b0a:f51f:e14e  prefixlen 64  scopeid 0x20<link>
    ether 00:d8:61:a9:da:ea  txqueuelen 1000  (Ethernet)
    RX packets 1910190  bytes 2676122114 (2.4 GiB)
    RX errors 0  dropped 0  overruns 0  frame 0
    TX packets 1091610  bytes 93109462 (88.7 MiB)
    TX errors 8  dropped 0 overruns 0  carrier 4  collisions 50576

lo: flags=73<UP,LOOPBACK,RUNNING>  mtu 65536
    inet 127.0.0.1  netmask 255.0.0.0
    inet6 ::1  prefixlen 128  scopeid 0x10<host>
    loop  txqueuelen 1000  (Local Loopback)
    RX packets 426238  bytes 327391145 (312.2 MiB)
    RX errors 0  dropped 0  overruns 0  frame 0
    TX packets 426238  bytes 327391145 (312.2 MiB)
    TX errors 0  dropped 0 overruns 0  carrier 0  collisions 0

veth60de6b9: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500
    inet6 fe80::d833:3eff:fe68:3a08  prefixlen 64  scopeid 0x20<link>
    ether da:33:3e:68:3a:08  txqueuelen 0  (Ethernet)
    RX packets 38566  bytes 2547730 (2.4 MiB)
    RX errors 0  dropped 0  overruns 0  frame 0
    TX packets 72584  bytes 107243713 (102.2 MiB)
    TX errors 0  dropped 0 overruns 0  carrier 0  collisions 0

";

    #[tokio::test]
    async fn parse_output() {
        let output = Output {
            status: ExitStatus::from_raw(0),
            stderr: Vec::new(),
            stdout: IP_OUTPUT.into(),
        };
        let mut real = IfConfig().parse_output(output).await.unwrap();
        let mut expected = vec![
            "192.168.0.11",
            "fe80::6954:9b0a:f51f:e14e",
            "172.23.0.1",
            "172.17.0.1",
            "fe80::42:79ff:fe2b:f5c3",
            "fe80::d833:3eff:fe68:3a08",
        ]
        .iter()
        .map(|ip| ip.parse::<IpAddr>())
        .filter_map(Result::ok)
        .collect::<Vec<IpAddr>>();

        // Sort for displaying sake
        real.sort();
        expected.sort();

        assert_eq!(
            real.len(),
            expected.len(),
            "Real vs. Expected lengths differ.\nReal    : {:?}\nExpected: {:?}",
            real,
            expected
        );

        for ip in expected {
            assert!(
                real.contains(&ip),
                "Results missing: {} from {:?}",
                ip,
                real
            );
        }
    }
}
