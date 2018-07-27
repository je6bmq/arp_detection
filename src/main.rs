extern crate pnet;
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes,EthernetPacket};
use pnet::util::MacAddr;

fn main() {
    let interfaces = datalink::interfaces();
    let own_mac_addr = MacAddr::new(u8::from_str_radix("XX", 16).unwrap(),
                                    u8::from_str_radix("XX", 16).unwrap(),
                                    u8::from_str_radix("XX", 16).unwrap(),
                                    u8::from_str_radix("XX", 16).unwrap(),
                                    u8::from_str_radix("XX", 16).unwrap(),
                                    u8::from_str_radix("XX", 16).unwrap());
    let interface = interfaces.iter()
        .filter(|ifd| {
            if let Some(addr) = ifd.mac {
                addr == own_mac_addr
            } else {
                false
            }
        })
        .next()
        .unwrap();

    let broadcast_addr = MacAddr::new(255u8, 255u8, 255u8, 255u8, 255u8, 255u8);
    println!("Waiting arp packets...");
    let mut rx = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => rx,
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("error occurred when creating the channel: {}", e),
    };
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                match (packet.get_ethertype(), packet.get_source(), packet.get_destination()) {
                    (EtherTypes::Arp, src, dst) if dst == broadcast_addr => {
                        println!("A ARP packet detected: from {}", src)
                    }
                    _ => continue,
                }
            }
            Err(e) => panic!("error occurred while readling: {}", e),
        }
    }

}
