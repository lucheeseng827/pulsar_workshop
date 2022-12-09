// Import the necessary classes from the raft-rs library
use raft::{NodeId, Config, ServerId, RawNode};

// Define the address and port for the leader node
let leader_id = NodeId::new("localhost:8000".to_string());

// Define the addresses and ports for the follower nodes
let follower_ids = [
    NodeId::new("localhost:8001".to_string()),
    NodeId::new("localhost:8002".to_string()),
    NodeId::new("localhost:8003".to_string())
];

// Create the configuration for the Raft cluster
let mut config = Config::new();
config.set_election_tick(10);
config.set_heartbeat_tick(3);

// Create the leader node
let leader_node = RawNode::new(&config, leader_id, follower_ids);

// Create the follower nodes
let follower_nodes = follower_ids.iter().map(|id| RawNode::new(&config, *id, follower_ids));

// Start the Raft cluster
leader_node.start();
for follower in follower_nodes {
    follower.start();
}
