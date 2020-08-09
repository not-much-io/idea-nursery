use clap::App;
use futures::join;
use lazy_static::lazy_static;
use log::LevelFilter;
use nety::net_interfaces::ifconfig::IfConfig;
use nety::net_interfaces::ip::Ip;
use nety::net_interfaces::{GetNetInterfaces, GetNetInterfacesResult, NetInterface};
use nety::public_ip::dig::Dig;
use nety::public_ip::{GetPublicIP, GetPublicIPResult};
use std::net::IpAddr;
use thiserror::Error;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let _matches = App::new("nety")
        .version("0.1")
        .author("kristo.koert@gmail.com")
        .about("A tool for gathering networking related information")
        .get_matches();

    let (public_ip_res, network_interfaces_res) = join!(get_public_ip(), get_net_interfaces());

    match public_ip_res {
        Ok(public_ip) => display_public_ip(public_ip),
        Err(err) => error!("{}", err),
    }

    match network_interfaces_res {
        Ok(net_interfaces) => display_network_interfaces(net_interfaces),
        Err(err) => error!("{}", err),
    }
}

#[derive(Error, Debug)]
pub enum NetyError {
    #[error("TODO")]
    NoGetNetInterfacesToolInstalled,
    #[error("TODO")]
    NoGetPublicIPToolInstalled,
}

lazy_static! {
    // Public ip tools in priority order
    static ref GET_PUBLIC_IP_TOOLS: [Box<dyn GetPublicIP + Sync>; 1] = [Box::new(Dig::default())];

    // Network interface tools in priority order
    static ref GET_NET_INTERFACES_TOOLS: [Box<dyn GetNetInterfaces + Sync>; 2] =
        [Box::new(Ip::default()), Box::new(IfConfig::default())];
}

async fn get_public_ip() -> GetPublicIPResult {
    if let Some(t) = GET_PUBLIC_IP_TOOLS.iter().find(|t| t.is_installed()) {
        return t.get_public_ip().await;
    }

    Err(NetyError::NoGetPublicIPToolInstalled.into())
}

async fn get_net_interfaces() -> GetNetInterfacesResult {
    if let Some(t) = GET_NET_INTERFACES_TOOLS.iter().find(|t| t.is_installed()) {
        return t.get_net_interfaces().await;
    }

    Err(NetyError::NoGetNetInterfacesToolInstalled.into())
}

fn display_public_ip(ip: IpAddr) {
    println!("Public IP: {}", ip)
}

fn display_network_interfaces(net_interfaces: Vec<NetInterface>) {
    println!("Network Interfaces:");
    for ni in net_interfaces {
        println!("  Name: {}", ni.name);

        if let Some(ipv4) = ni.ipv4 {
            println!("  ipv4: {}", ipv4)
        }

        if let Some(ipv6) = ni.ipv6 {
            println!("  ipv6: {}", ipv6)
        }

        println!();
    }
}
