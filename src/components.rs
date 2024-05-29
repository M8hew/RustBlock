use std::rc::Rc;
use std::option::Option;
use crate::messages::Messages;

use rand_distr::num_traits::real;
use serde::Serialize;
use dslab::{EventHandler, SimulationContext, Id};


#[derive(Serialize, Clone)]
pub struct Message {
    pub sender_addr: Id,
    pub message_payload: Messages,
}

#[derive(Serialize, Clone)]
pub struct Block {
    pub block_id: u32,
    pub height: u32,
    pub prev_block: Option<Rc<Block>>,

    pub txs: Vec<Tx>,
}

#[derive(Serialize, Clone)]
pub struct Tx {
    pub tx_id : u64,
    pub tx_fee: f64,
}

pub struct InitialState {
    pub known_hosts: Vec<Id>,
    pub known_transactions: Vec<Tx>,
    pub cur_block: Rc<Block>,
}

#[derive(Serialize, Clone, Copy)]
pub struct NodeType {
    pub full_blocks: bool,
}

#[derive(Serialize, Clone)]
pub struct PeerInfo {
    pub type_: NodeType,
}


// message type
// message payload
// if mathc type => call_func with payload


// init functions
// event_handlers
// 
pub trait Node : EventHandler {
    // auto_init for topology initialization?
    fn new_node(ctx : SimulationContext, inital_state : InitialState) -> Self;
    fn node_name() -> String;
    
    // custom node init
    fn init(&mut self);
    // connect to persons
    // try mine block


    // connection request handler
    fn connect(&mut self, request_from: Id, peer_info: PeerInfo);

    // handles tx requested
    fn req_tx_handler(&self, request_from: Id);
    // handles tx proposed
    fn prop_tx_handler(&self, request_from: Id, txs: Vec<Tx>);

    // get_addr request handler
    fn get_addr(&self, request_from: Id);
    // get_addr response handler
    fn get_addr_resp(&self, request_from: Id, addrs: Vec<Id>);

    // get_block request handler
    fn get_block(&self, request_from: Id, last_known_height: u32);
    // get_block response handler
    fn get_block_resp(&self, request_from: Id);


    // get block headers ?
    // block_gen?
    // check_Tx (for pruned)?
    // 

    // for self given events
    fn custom(&self, request_from: Id);
    // stat count
    fn middlerware(&self, msg: Message);
}