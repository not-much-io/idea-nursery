use std::collections::HashMap;
use std::net::IpAddr;
use std::process::{Command, Output};

use crate::net_interfaces::{
    GetNetInterfaces, GetNetInterfacesError, GetNetInterfacesResult, NetInterface,
};

use nursery_prelude::library_prelude::*;

// https://man7.org/linux/man-pages/man8/ip.8.html
pub struct Ip();

impl Ip {
    pub fn new() -> Ip {
        Ip {}
    }
}

impl Default for Ip {
    fn default() -> Self {
        Ip::new()
    }
}

#[async_trait]
impl GetNetInterfaces for Ip {
    async fn get_net_interfaces(&self) -> GetNetInterfacesResult {
        self.parse_output(self.call().await?).await
    }
}

#[async_trait]
impl CLIProgram<GetNetInterfacesResult> for Ip {
    fn name(&self) -> &str {
        "ip"
    }

    async fn call(&self) -> Result<Output> {
        Ok(Command::new(self.name())
            .arg("-o")
            .arg("addr")
            .arg("list")
            .output()?)
    }

    async fn parse_output(&self, output: Output) -> GetNetInterfacesResult {
        // NOTE: This is done by parsing byte by byte and running in async just for fun.
        //       The same can be done shorter / simpler via something like s.split("\n").split(" ").map(f) and a loop.
        //       Performance difference doesn't matter since the input will always be "tiny".
        let mut line = Vec::new();
        let mut parsing_handles = Vec::new();
        for b in output.stdout.into_iter() {
            line.push(b);
            if b == b'\n' {
                parsing_handles.push(tokio::spawn(parse_line(line.to_vec())));
                line.clear();
            }
        }

        let name_address_pairs = futures::future::join_all(parsing_handles)
            .await
            .into_iter()
            .collect::<Result<Vec<(Vec<u8>, Vec<u8>)>, _>>()?;

        let mut net_interfaces: HashMap<String, NetInterface> = HashMap::new();
        for (name_bs, ip_bs) in name_address_pairs {
            if name_bs.is_empty() {
                return Err(GetNetInterfacesError::NoNameForInterfaceFound().into());
            }
            if ip_bs.is_empty() {
                return Err(GetNetInterfacesError::NoAddrForInterfaceFound().into());
            }

            let name = String::from_utf8(name_bs)?;
            let ip_addr = String::from_utf8(ip_bs)?.parse::<IpAddr>()?;

            match net_interfaces.get_mut(&name) {
                None => {
                    let ni = NetInterface::new_with_single_ip(&name.as_str(), &ip_addr);
                    net_interfaces.insert(name, ni);
                }
                Some(ni) => {
                    ni.set_ip(&ip_addr);
                }
            }
        }

        Ok(net_interfaces
            .values()
            .cloned()
            .sorted_by(|a, b| Ord::cmp(&a.name, &b.name))
            .collect::<Vec<NetInterface>>())
    }
}

async fn parse_line(line: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let mut line_iter = line.into_iter();
    (parse_name(&mut line_iter), parse_ip(&mut line_iter))
}

fn parse_name(line_iter: &mut dyn Iterator<Item = u8>) -> Vec<u8> {
    let mut name = Vec::new();

    let mut prev_byte = 0;
    let mut collecting = false;
    for curr_byte in line_iter {
        if collecting && curr_byte == b' ' {
            break;
        }

        if prev_byte == b' ' && curr_byte != b' ' {
            collecting = true;
        }

        if collecting {
            name.push(curr_byte);
        }

        prev_byte = curr_byte;
    }

    name
}

fn parse_ip(line_iter: &mut dyn Iterator<Item = u8>) -> Vec<u8> {
    let mut ip = Vec::new();

    let mut prev_byte = 0;
    let mut collecting = false;
    for curr_byte in line_iter {
        if collecting && curr_byte == b'/' {
            break;
        }

        if collecting {
            ip.push(curr_byte);
        }

        // inet or inet6
        if (prev_byte == b't' || prev_byte == b'6') && curr_byte == b' ' {
            collecting = true;
        }

        prev_byte = curr_byte;
    }

    ip
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    const IP_OUTPUT: &str = r"1: lo    inet 127.0.0.1/8 scope host lo\       valid_lft forever preferred_lft forever
1: lo    inet6 ::1/128 scope host \       valid_lft forever preferred_lft forever
2: enp34s0    inet 192.168.0.11/24 brd 192.168.0.255 scope global dynamic noprefixroute enp34s0\       valid_lft 2180sec preferred_lft 2180sec
2: enp34s0    inet6 fe80::6954:9b0a:f51f:e14e/64 scope link noprefixroute \       valid_lft forever preferred_lft forever
4: docker0    inet 172.17.0.1/16 brd 172.17.255.255 scope global docker0\       valid_lft forever preferred_lft forever
4: docker0    inet6 fe80::42:f3ff:fe8b:ca5c/64 scope link \       valid_lft forever preferred_lft forever
5: br-60984024090a    inet 172.18.0.1/16 brd 172.18.255.255 scope global br-60984024090a\       valid_lft forever preferred_lft forever
7: veth5e001f8    inet6 fe80::1442:1ff:feb9:41b5/64 scope link \       valid_lft forever preferred_lft forever
";

    #[tokio::test(core_threads = 6)]
    async fn test_parse_output() {
        let output = Output {
            status: ExitStatus::from_raw(0),
            stderr: Vec::new(),
            stdout: IP_OUTPUT.into(),
        };

        let real = Ip().parse_output(output).await.unwrap();
        let expected = vec![
            (
                "br-60984024090a",
                Some("172.18.0.1".parse::<IpAddr>().unwrap()),
                None,
            ),
            (
                "docker0",
                Some("172.17.0.1".parse::<IpAddr>().unwrap()),
                Some("fe80::42:f3ff:fe8b:ca5c".parse::<IpAddr>().unwrap()),
            ),
            (
                "enp34s0",
                Some("192.168.0.11".parse::<IpAddr>().unwrap()),
                Some("fe80::6954:9b0a:f51f:e14e".parse::<IpAddr>().unwrap()),
            ),
            (
                "lo",
                Some("127.0.0.1".parse::<IpAddr>().unwrap()),
                Some("::1".parse::<IpAddr>().unwrap()),
            ),
            (
                "veth5e001f8",
                None,
                Some("fe80::1442:1ff:feb9:41b5".parse::<IpAddr>().unwrap()),
            ),
        ];

        for (i, (name, ip_v4, ip_v6)) in expected.iter().enumerate() {
            let net_interface = real.get(i).unwrap();

            assert_eq!(*name, net_interface.name);

            assert_eq!(*ip_v4, net_interface.ipv4);
            assert_eq!(*ip_v6, net_interface.ipv6);
        }
    }

    #[tokio::test(core_threads = 6)]
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
