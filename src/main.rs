use std::cell::RefCell;
use std::rc::Rc;

use dslab::{Simulation, Id};
use rustblock::_full_node::BitcoinFullNode;

use rustblock::common::{create_chain, gen_peers, connect_with_uniform_prob};
use rustblock::driver::Driver;
use rustblock::_structs_2::{InitialState, Node};

use log::{debug, info};
use env_logger;
// use rustblock::node::BitcoinNode;

fn init_nodes<T: Node + 'static>(
    sim : &mut Simulation,
    node_cnt : u32,
    mut cntr: u32,
    is_connected: fn(u32, u32)->bool, // change to generating edge list
    ) -> Vec<Id> {

    let mut  node_ids = Vec::new(); 
    let latest_block = create_chain();
    for _i in 1..=node_cnt {
        let node_name = format!("{} {}", T::node_name(), cntr);
        let node_ctx = sim.create_context(node_name.clone());
        let peers = gen_peers(cntr, 1, node_cnt, is_connected);

        let mut new_node = T::new_node(node_ctx, InitialState{
            known_hosts: peers,
            known_transactions: Vec::new(),
            cur_block: latest_block.clone(),
        });
        new_node.init();
        node_ids.push(sim.add_handler(node_name.clone(), Rc::new(RefCell::new(new_node))));            
        cntr += 1;
    }
    node_ids
}

const FULL_NODE_CNT: u32 = 20;

fn main() {
    // Initialize the logger
    env_logger::init();
    info!("Logging level -- info");
    debug!("Loggin level -- debug");

    let mut sim = Simulation::new( 42);
    let mut driver_ = Driver::new(sim.create_context("Driver"));

    let cntr = 1;
    let ids = init_nodes::<BitcoinFullNode>(&mut sim, FULL_NODE_CNT, cntr, connect_with_uniform_prob);

    driver_.add_id(ids);

    info!("Simulation started");

    driver_.run_scenario(&mut sim);

    info!("Simulation finished");
    
    // drop node states
}
