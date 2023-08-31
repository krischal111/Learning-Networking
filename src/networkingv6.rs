use std::net::Ipv6Addr;

use crate::bit_utils::popcount;

pub trait IpAddrTools {
    fn count_contiguous_ones(self) -> usize;
    fn mask(self, mask:Self) -> Self;
}

impl IpAddrTools for Ipv6Addr {
    fn count_contiguous_ones(self) -> usize {
        popcount::<u128>(self.into())
    }
    fn mask(self, mask:Self) -> Self {
        let ip : u128 = self.into();
        let mask : u128 = self.into();
        let result = ip & mask;
        return result.into();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Interface {
    IpAddr(Ipv6Addr),
    Port(u64),
}

#[derive(Debug, Clone)]
pub struct Route {
    pub destination: Ipv6Addr,
    pub mask : Ipv6Addr,
    pub next_hop :Interface,
}

impl Route {
    // checks the match of this ipaddr with the route
    pub fn matches(&self, ipaddr: Ipv6Addr) -> bool {
        ipaddr.mask(self.mask) == self.destination
    }
}

#[derive(Debug)]
struct RoutingTable {
    name : String,
    table : Vec<Route>,
}

impl RoutingTable {
    // finds the best matching address from the routing table
    pub fn find_best_route(&self, ipaddr:Ipv6Addr) -> Option<&Route> {
        self.table
            .iter()
            .filter(|route| route.matches(ipaddr))
            .max_by_key(|route| route.mask.count_contiguous_ones())
    }
    pub fn find_next_hop(&self, ipaddr: Ipv6Addr) -> Option<Interface> {
        if let Some(route) = self.find_best_route(ipaddr) {
            return Some(route.next_hop.clone());
        } else {
            return None;
        }

    }
}

// #[test]
pub fn check_routing() {
    let my_routing_table = RoutingTable {
        name: "Krischal's router".into(),
        table : vec![
            Route {destination: 0.into(), mask: (u128::MAX).into() , next_hop: Interface::Port(30)},
        ],
    };
    let my_ip_addr = 0.into();

    let my_best_route = my_routing_table.find_best_route(my_ip_addr);
    let my_hop = my_routing_table.find_next_hop(my_ip_addr);
    println!("The routing table is {my_routing_table:#?}");
    println!();
    println!("The next hop for {my_ip_addr:?} is {my_hop:?}");
    println!();
    println!("The best route for {my_ip_addr:?} is {my_best_route:#?}");
}

#[test]
pub fn routing_works() {
    check_routing();
}