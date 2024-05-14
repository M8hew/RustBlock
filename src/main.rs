use std::cell::RefCell;
use std::rc::Rc;

use dslab::Simulation;

use rustblock::driver::{self, Driver};
use rustblock::node::BitcoinNode;

fn main() {
    let mut sim = Simulation::new(42);

    let mut _driver = Driver::new(sim.create_context("Driver"));

    let mut node_hanlders = Vec::new();

    let node_cnt = 7;
    for i in 1..=node_cnt {
        let node_name = format!("node {}", i);
        let cur_node_handler = Rc::new(RefCell::new(BitcoinNode::new(
            sim.create_context(node_name.clone()),
        )));
        let cur_node_id = sim.add_handler(node_name.clone(), cur_node_handler.clone());

        node_hanlders.push(cur_node_handler);
        _driver.add_id(cur_node_id);
    }

    _driver.run_trigger();

    sim.step_until_no_events();
    // loop for some events, collect stats and call callbacks on events

    for node in node_hanlders {
        println!("{:?}", node);
    }
}
