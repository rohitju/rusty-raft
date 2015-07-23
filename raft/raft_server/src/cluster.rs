use std::net::SocketAddr;
use ini::Ini;

pub struct Cluster {
    leader: SocketAddr,
    nodes: Vec<SocketAddr>
}

impl Cluster {

    pub fn new(id: &str) -> Cluster {
        let cluster: Cluster = read_cluster_config(id);
        //Cluster { leader:default_addr, nodes:Vec::<SocketAddr>::new() }
        return cluster;
    }

    /// This function attempts to read the latest cluster config if present from 
    /// the cluster config file. On each committed cluster config entry this file
    /// has to be written. If file does not exist (or unable to read) send new config 
    /// with self as the leader.
    fn read_cluster_config(id: &str) -> Cluster {
        let cluster_file = "/var/cluster_config_" + id;
        let cluster = match Ini::load_from_file(cluster_file) {
            Ok(conf) => {
                let mut section = conf.section(Some("Leader")).unwrap();
                let leader_addr = section.get("leader").unwrap();
                let leader: SocketAddr = leader_addr.parse().unwrap();
                section = conf.section(Some("Nodes")).unwrap();

                /// Try to get nodes of cluster
                let mut nodes: Vec::<SocketAddr>::new();
                for (key, value) in section.iter() {
                        let addr: SocketAddr = value.parse().unwrap();
                        nodes.push(addr);
                    
                }
                Cluster { leader: leader, nodes: nodes }
            }
            Err(e) => {
                /// Unable to create cluster config from file. Send new config.
                println!("Failed to read cluster config file with error {}.", e);
                let default_addr: SocketAddr = "127.0.0.1:1234".parse().unwrap();
                Cluster { leader: default_addr, nodes: Vec::<SocketAddr>::new() }
        }
        return cluster;
    }

    fn set_leader(&mut self, leader: String) {
        println!("Trying to set leader as {}", leader.to_string());
        let addr: SocketAddr = leader.parse().unwrap();
        self.leader = addr;
        println!("set leader of cluster");
    }

    pub fn get_leader(&mut self) -> SocketAddr {
        self.leader
    }

    fn add_node(&mut self, address: String) {
        let addr: SocketAddr = address.parse().unwrap();
        self.nodes.push(addr);
    }

    fn add_nodes(&mut self, addresses: &Vec<String>) {
        for s in addresses {
            let addr: SocketAddr = s.parse().unwrap();
            self.nodes.push(addr);
        }
        println!("set nodes in cluster");
    }

    pub fn get_nodes(&mut self) -> &Vec<SocketAddr>{
        &self.nodes
    }
    
    
}
