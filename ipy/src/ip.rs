use crate::{GetPrivateIP, IPV4_LOCALHOST, IPV6_LOCALHOST};
use anyhow::Result;
use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use std::net::IpAddr;
use std::process::{Command, Output};
use toolbox_rustbase::CLIProgram;

// https://man7.org/linux/man-pages/man8/ip.8.html
struct Ip();

lazy_static! {
    // Capture the ip definition from ip-s output.
    // Contains three groups:
    //  (inet6|inet)    The network interface definition beginning
    //  (.*?)           The actual IP - this is the interesting group
    //  /               CIDR notation / slash notation
    static ref RE: Regex = Regex::new(r"(inet6|inet) (.*?)/").unwrap();
}

#[async_trait]
impl CLIProgram<Result<Vec<IpAddr>>> for Ip {
    fn name(&self) -> &str {
        "ip"
    }

    async fn call(&self) -> Result<Output> {
        Ok(Command::new(self.name()).arg("addr").output()?)
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
impl GetPrivateIP for Ip {
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
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host 
       valid_lft forever preferred_lft forever
2: enp34s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 00:d8:61:a9:da:ea brd ff:ff:ff:ff:ff:ff
    inet 192.168.0.11/24 brd 192.168.0.255 scope global dynamic noprefixroute enp34s0
       valid_lft 3353sec preferred_lft 3353sec
    inet6 fe80::6954:9b0a:f51f:e14e/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever
3: br-b83013461f0c: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default 
    link/ether 02:42:5d:8c:83:bc brd ff:ff:ff:ff:ff:ff
    inet 172.23.0.1/16 brd 172.23.255.255 scope global br-b83013461f0c
       valid_lft forever preferred_lft forever
4: docker0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default 
    link/ether 02:42:79:2b:f5:c3 brd ff:ff:ff:ff:ff:ff
    inet 172.17.0.1/16 brd 172.17.255.255 scope global docker0
       valid_lft forever preferred_lft forever
    inet6 fe80::42:79ff:fe2b:f5c3/64 scope link 
       valid_lft forever preferred_lft forever
26: veth60de6b9@if25: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master docker0 state UP group default 
    link/ether da:33:3e:68:3a:08 brd ff:ff:ff:ff:ff:ff link-netnsid 1
    inet6 fe80::d833:3eff:fe68:3a08/64 scope link 
       valid_lft forever preferred_lft forever
";

    #[tokio::test]
    async fn parse_output() {
        let output = Output {
            status: ExitStatus::from_raw(0),
            stderr: Vec::new(),
            stdout: IP_OUTPUT.into(),
        };
        let mut real = Ip().parse_output(output).await.unwrap();
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
