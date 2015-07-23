extern crate ini;
use ini::ini;

use log;
use cluster;
use message;

/// Consensus state of the server. The various variables are protected 
/// by a monitor (sort of).
/// Threads access the functions through the monitor.
/// 
/// Consists of persistent state 
/// as well as volatile state.

pub struct Server {
    /// Persistent state
    id: String,
    log: log::Log,
    curr_term: u32,
    voted_for: u32,

    /// Volatile state.
    raft_cluster: cluster::Cluster,
    commit_index: u32,
    last_applied: u32,
    role: Role,
    state: State,

    /// Leader specific state
    next_index: Vec<u32>,
    match_index: Vec<u32>
}

impl Server {
    pub fn new(filepath: String, bootstrap: bool, address: String, id: String) -> Server {
        let log = log::Log::new(filepath.to_string());
        let mut curr_term;
        let mut curr_state;
        if (bootstrap)
        {
            curr_state = State::ACTIVE;
            curr_term = 1;
        }
        else
        {
            curr_state = State::STANDBY;
            curr_term = 0;
        }
        let raft_cluster: cluster::Cluster = cluster::Cluster::new(id.to_string());
        Server { id:id, log:log, role:Role::LEADER, state:curr_state, curr_term:curr_term, raft_cluster:raft_cluster} 
    }
    
    pub fn set_cluster_config(&mut self, config: &message::ClusterConfig) {
        if (self.state == State::ACTIVE)
        {
            /// TODO: Protect with mutex
            let leader = config.get_leader();
            let machines = config.get_machines();
            self.raft_cluster.set_leader(leader.to_string());
            self.raft_cluster.add_nodes(&machines.to_vec());
        }
        /// If we are leader send the cluster configuration to all the other nodes
        if (self.role == Role::LEADER)
        {
            
        }
    }

    fn read_configuration() -> (String, String) {

    }
}

#[derive(PartialEq)]
pub enum Role {
    FOLLOWER = 1,
    CANDIDATE = 2,
    LEADER = 3,
}

#[derive(PartialEq)]
pub enum State {
    STANDBY = 1,
    ACTIVE = 2,
}
