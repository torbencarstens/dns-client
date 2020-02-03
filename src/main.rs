extern crate clap;
extern crate dnsclient;

use dnsclient::sync::DNSClient;
use dnsclient::UpstreamServer;
use std::io;
use std::net::{SocketAddrV4, Ipv4Addr};
use std::str::FromStr;
use clap::{App, Arg};

fn main() -> io::Result<()> {
    let app = App::new("dns-client")
        .version("0.0.2")
        .about("Lookup of a given website via different DNS servers (default is 8.8.8.8 and 1.1.1.1)")
        .author("Torben Carstens")
        .arg(Arg::with_name("target")
            .required(true)
            .help("domain name to lookup")
            .takes_value(true))
        .arg(Arg::with_name("upstreams")
            .short("-u")
            .long("upstreams")
            .help("List of DNS servers (comma separated)")
            .value_name("upstreams")
            .default_value("8.8.8.8,1.1.1.1")
            .takes_value(true));
    let matches = app.get_matches();

    // this unwrap is safe, since we require it in clap
    let target = matches.value_of("target").unwrap();

    // this unwrap is safe, since we provide clap with a default
    let upstreams: Vec<&str> = matches.value_of("upstreams").unwrap().split(",").collect();
    let valid_upstreams = upstreams
        .into_iter()
        .filter_map(|x|
            Some(SocketAddrV4::new(Ipv4Addr::from_str(x).ok()?, 53)))
        .map(|x|
            UpstreamServer::new(x));

    for upstream in valid_upstreams {
        // this unwrap is safe, since split always returns at least one item
        let upstream_address = upstream.clone().addr.to_string().split(":").next().unwrap().to_string();

        match DNSClient::new(vec![upstream]).query_a(target) {
            Ok(ips) => {
                let ips: String = ips
                    .iter()
                    .map(|x|
                        x.to_string())
                    .collect::<Vec<String>>()
                    .join(" -> ");
                if ips.is_empty() {
                    println!("No record returned from: {}", upstream_address)
                } else {
                    println!("{}: {}", upstream_address, ips)
                }

            },
            Err(e) => {
                eprintln!("Error while retrieving DNS record from {}: {}", upstream_address, e)
            },
        }
    }

    Ok(())
}
