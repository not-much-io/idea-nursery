use crate::net_interfaces::{GetNetInterfaces, GetNetInterfacesResult, NetCLIProgram};
use anyhow::Result;
use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use std::process::{Command, Output};
use toolbox_rustbase::CLIProgram;

// https://man7.org/linux/man-pages/man8/ip.8.html
struct Ip();

impl GetNetInterfaces for Ip {}

#[async_trait]
impl CLIProgram<GetNetInterfacesResult> for Ip {
    fn name(&self) -> &str {
        "ip"
    }

    async fn call(&self) -> Result<Output> {
        Ok(Command::new(self.name()).arg("addr").output()?)
    }

    async fn parse_output(&self, output: Output) -> GetNetInterfacesResult {
        self.parse_output_to_net_interfaces(output).await
    }
}

impl NetCLIProgram for Ip {
    fn get_regex(&self) -> &Regex {
        &RE
    }
}

lazy_static! {
    /// Regex to get all the data from the ip command output
    /// TODO: Pretty hard to grok, some way to simplify, explain, format?
    static ref RE: Regex = Regex::new(r#": (?P<interface_name>.*?): (?:[\S\s]*?inet (?P<interface_ip_v4>.*?)/){0,1}(?:[\S\s]*?(?:\n[0-9]|inet6 (?P<interface_ip_v6>.*?)/))"#).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

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
    async fn test_parse_output() {
        let output = Output {
            status: ExitStatus::from_raw(0),
            stderr: Vec::new(),
            stdout: IP_OUTPUT.into(),
        };

        let real = Ip().parse_output(output).await.unwrap();
        let expected = vec![
            (
                "lo",
                Some("127.0.0.1".parse::<IpAddr>().unwrap()),
                Some("::1".parse::<IpAddr>().unwrap()),
            ),
            (
                "enp34s0",
                Some("192.168.0.11".parse::<IpAddr>().unwrap()),
                Some("fe80::6954:9b0a:f51f:e14e".parse::<IpAddr>().unwrap()),
            ),
            (
                "br-b83013461f0c",
                Some("172.23.0.1".parse::<IpAddr>().unwrap()),
                None,
            ),
            (
                "docker0",
                Some("172.17.0.1".parse::<IpAddr>().unwrap()),
                Some("fe80::42:79ff:fe2b:f5c3".parse::<IpAddr>().unwrap()),
            ),
            (
                "veth60de6b9@if25",
                None,
                Some("fe80::d833:3eff:fe68:3a08".parse::<IpAddr>().unwrap()),
            ),
        ];

        for (i, (name, ip_v4, ip_v6)) in expected.iter().enumerate() {
            let net_interface = real.get(i).unwrap();

            assert_eq!(*name, net_interface.name);

            assert_eq!(*ip_v4, net_interface.ipv4);
            assert_eq!(*ip_v6, net_interface.ipv6);
        }
    }

    #[tokio::test]
    async fn test_actual_call() {
        let ip = Ip();

        assert!(ip.is_installed(), "ip not installed in environment");

        let interfaces = ip.get_net_interfaces().await.unwrap();

        assert!(
            !interfaces.is_empty(),
            "No network interfaces at all in this environment. Required for test.",
        );
        for interface in interfaces {
            assert_ne!(interface.name, "", "Network interface name empty.");
        }
    }
}
