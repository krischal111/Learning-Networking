/// I have a question:
///  If NAT looks into the TCP or UDP or any protocol header, then
///  isn't it working in the 4th layer?
/// -> My own answer is, it's just looking into it, (It shouldn't do even that)
///    The real Layer 4 would not only look into it, but also modify it,
///     it would be able to work in it, create it, talk to others, etc.
/// 
/// Further, this is a very crude implementation:
/// A real router has 2^16 space for NAT: each one corresponding to a port
///     -> this saves space for storing port, but allocates full space for all ports
///       -> It probably also needs to know if the port is allcoated, so single bit could be used.
/// The real reason that routers use that is, it gives indexing (Works in O(1) time), for the cost of just 64 thousand entries
/// And since, they do not need to store port, they will store ipv4_addr and the port (32 + 16 bits) there.
/// The router would have just a single ip-address they can give.
/// The searching of next free port could take O(n) time, but it can easily be pipelined.
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RandomTransportPacket {
    // computer : u16, // This should be on perhaps Data Link Layer, so I removed it
    time_to_live : Duration,
    source_ip : Ipv4Addr,
    destination_ip : Ipv4Addr,
    source_port : u16,
    destination_port: u16,

    data : String, // The upper part should be header, and bottom part should be used separately
}

#[derive(Debug)]
pub struct NatEntry {
    pub source_ip: Ipv4Addr,
    pub source_port : u16,
    pub computer : u16,
    pub mangled_port : u16,
    pub mapped_on_time : Instant,
    pub time_to_live : Duration,
}

#[derive(Debug)]
pub struct NatTable {
    pub name : String,
    pub translated_addr : Ipv4Addr,
    pub table : Vec<NatEntry>,
}

impl NatTable {
    pub fn has_available_port(&self, port: u16) -> bool {
        self.table
            .iter()
            .find(|entry| entry.mangled_port == port)
            .is_none()
    }
    pub fn extract_available_port(&self) -> Option<u16> {
        (0..u16::MAX)
            .filter(|&port| self.has_available_port(port))
            .next()
    }
    pub fn give_me_a_port(&mut self, my_ip : Ipv4Addr, my_port: u16, me: u16, duration: Duration) -> Option<(Ipv4Addr, u16)> {
        // I am a table that will give this my computer a port
        let available_port = 
        if let Some(port) = self.extract_available_port(){
            // println!("I have available port as {port}");
            // If I have an available port, I give that
            port
        } else {
            // If I don't have then I will prune unnecessary ports
            self.prune_unnecessary_ports();
            // Then again, when I try to assign a port
            // If it fails still, the none is propagated outwards
            self.extract_available_port()?
        };

        let entry = NatEntry {
            source_ip : my_ip,
            source_port : my_port,
            mangled_port : available_port,
            computer : me,
            mapped_on_time : Instant::now(),
            time_to_live : duration,
        };

        self.table.push(entry);
        Some((self.translated_addr, available_port))
    }

    pub fn prune_unnecessary_ports(&mut self) {
        let new_now = Instant::now();
        self.table
            .retain(|table| new_now.duration_since(table.mapped_on_time) < table.time_to_live );
    }

    pub fn found_on_nat(&self, ip_addr: Ipv4Addr, port: u16) -> Option<&NatEntry> {
        self.table
            .iter()
            .find(|table| table.source_ip == ip_addr && table.source_port == port)
    }

    pub fn translate_incoming(&self, mut packet: RandomTransportPacket) -> Option<(RandomTransportPacket, u16)> {
        let nat_entry = 
        self.table
            .iter()
            .find(|table| table.mangled_port == packet.destination_port)?;
        packet.destination_ip = nat_entry.source_ip;
        packet.destination_port = nat_entry.source_port;
        Some((packet, nat_entry.computer))
    }

    pub fn translate_outgoing(&mut self, mut packet: RandomTransportPacket, computer: u16) -> Option<RandomTransportPacket> {
        if let Some(nat_entry) = self.found_on_nat(packet.source_ip, packet.source_port) {
            packet.source_ip = self.translated_addr;
            packet.source_port = nat_entry.mangled_port;
        }
        let (ip, port) = self.give_me_a_port(packet.source_ip, packet.source_port, computer , packet.time_to_live)?;
        packet.source_ip = ip;
        packet.source_port = port;
        Some(packet)
    }
}


pub fn test_translation_outgoing() {
    let my_packet = RandomTransportPacket {
        time_to_live: Duration::from_secs(20),
        source_ip : "10.100.1.1".parse().unwrap(),
        destination_ip : "192.168.1.1".parse().unwrap(),
        source_port : 8090,
        destination_port : 80,

        data : "K xa bro, haal khabar?".to_string(),
    };

    let mut my_nattable = NatTable {
        name : "Krischal's NAT".to_string(),
        translated_addr : "103.5.150.9".parse().unwrap(),
        table : vec![]
    };

    println!("\nTesting outgoing NAT\n");
    let new_packet = my_nattable.translate_outgoing(my_packet.clone(), 12);
    println!("Original packet was: \n {my_packet:#?}");
    println!("New translated packet is: \n {new_packet:#?}");

}

pub fn test_translation_incoming() {
    let my_packet = RandomTransportPacket {
        time_to_live: Duration::from_secs(20),
        source_ip : "10.100.1.1".parse().unwrap(),
        destination_ip : "192.168.1.1".parse().unwrap(),
        source_port : 8090,
        destination_port : 120,

        data : "K xa bro, haal khabar?".to_string(),
    };

    let mut my_nattable = NatTable {
        name : "Krischal's NAT".to_string(),
        translated_addr : "192.168.1.1".parse().unwrap(),
        table : vec![
            NatEntry {
                source_ip : "103.5.150.9".parse().unwrap(),
                source_port : 80,
                computer : 12,
                mangled_port : 120,
                mapped_on_time : Instant::now(),
                time_to_live : Duration::from_secs(30),
            },
        ]
    };

    println!("\nTesting incoming NAT\n");
    let new_packet = my_nattable.translate_incoming(my_packet.clone());
    println!("Original packet was: \n {my_packet:#?}");
    if let Some((packet, computer)) = new_packet {
        println!("New translated packet is: \n {packet:#?} ");
        println!("The packet will be translated to the computer {computer}");
    } else {
        println!("The translated packet is {new_packet:?}");
    }

}

#[test]
fn translation_works() {
    test_translation_outgoing();
    test_translation_incoming();
}