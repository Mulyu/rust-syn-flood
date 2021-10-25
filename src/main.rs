use std::net::{IpAddr, Ipv4Addr};
use pnet;
use pnet::datalink::Config;
use pnet::datalink::channel;
use pnet::datalink::interfaces;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::tcp::{MutableTcpPacket, TcpFlags};
use rand::Rng;
use pnet::datalink::Channel::Ethernet;

const IPV4_HEADER_LEN: usize = 20;
const TCP_HEADER_LEN: usize = 32;

fn main() {
    let all_interfaces = interfaces();

    let default_interface = all_interfaces
     .iter()
     .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty())
     .expect("Error while finding default interface.");
    let (mut sender, _) = match channel(default_interface, Config::default()) {
        Ok(Ethernet(s, r)) => (s, r),  
        
        _ => panic!("No channel")
    };
    sender.build_and_send(1000, IPV4_HEADER_LEN + TCP_HEADER_LEN, &mut |buffer| build_syn_packet(buffer));
}

fn build_syn_packet(buffer: &mut [u8]) -> () {
    let mut rng = rand::thread_rng();
    let mut ip = MutableIpv4Packet::new(&mut buffer[..]).unwrap();
    ip.set_source(Ipv4Addr::new(127, 0, 0, 1));
    ip.set_destination(Ipv4Addr::new(127, 0 ,0, 1));
    let mut tcp = MutableTcpPacket::new(&mut buffer[IPV4_HEADER_LEN..]).unwrap();
    tcp.set_flags(TcpFlags::SYN);
    tcp.set_source(rng.gen());
    tcp.set_destination(rng.gen());
}