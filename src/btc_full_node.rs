use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::string::String;
use std::cmp::min;

use dslab::log_debug;
use dslab::{cast, log_info, Event, Id, SimulationContext, EventHandler};

use crate::components::{NodeType, PeerInfo, Tx, Message, Node, InitialState, Block};
use crate::messages::Messages; 
use crate::common::calculate_delay;

pub struct BitcoinFullNode {
    id: Id,
    type_: NodeType,
    ctx: Rc<RefCell<SimulationContext>>,

    peers: HashMap<Id, PeerInfo>,
    mempool: Vec<Tx>,

    known_hosts: HashSet<Id>,
    latest_block: Rc<Block>,  

    // alternative branches   
}

impl EventHandler for BitcoinFullNode {
    fn on(&mut self, event: Event) {
        cast! {match event.data{
            Message{sender_addr, message_payload} => {
                match message_payload {
                    Messages::Connect { info } => {
                        log_debug!(self.ctx.borrow(), "Node {} conn request", sender_addr);
                        self.connect(sender_addr, info);
                    },
                    Messages::Txs { Txs } => {
                        log_debug!(self.ctx.borrow(), "Got {} txs from {}", Txs.len(), sender_addr);
                        self.prop_tx_handler(sender_addr, Txs);
                    }
                }
            }
        }}
    }
}

impl Node for BitcoinFullNode {
    fn new_node(ctx : SimulationContext, inital_state : InitialState) -> Self {
        BitcoinFullNode {
            id: ctx.id(),
            ctx: Rc::new(RefCell::new(ctx)),
            type_: NodeType{full_blocks: true},

            peers: HashMap::new(),
            mempool: inital_state.known_transactions,
            known_hosts: HashSet::from_iter(inital_state.known_hosts.iter().cloned()),

            latest_block: inital_state.cur_block,
        }
    }

    fn node_name() -> String {
        "BtsFullNode".to_string()
    }

    fn init(&mut self) {
        for id in self.known_hosts.iter() {
            self.ctx.borrow_mut().emit(
                Message{
                    sender_addr: self.id.clone(),
                    message_payload: Messages::Connect{info: PeerInfo{type_: self.type_}}
                },
                id.clone(),
                calculate_delay(self.id, id.clone()),
            );
        }
    }

    fn connect(&mut self, request_from: Id, peer_info: PeerInfo) {
        if self.known_hosts.contains(&request_from) {
            self.known_hosts.remove(&request_from);
        }
        if !self.peers.contains_key(&request_from) {
            self.peers.insert(request_from.clone(), peer_info);

            self.ctx.borrow_mut().emit(
                Message{
                    sender_addr: self.id,
                    message_payload: Messages::Connect{info: PeerInfo{type_: self.type_}}
                },
                request_from.clone(),
                calculate_delay(self.id, request_from.clone()),
            );
        }
    }

    fn prop_tx_handler(&self, request_from: Id, txs: Vec<Tx>) {
        for tx in txs {
            self.mempool.push(tx.clone());
        }

        for id in self.known_hosts {
            self.ctx.borrow_mut().emit(
                Message{
                    sender_addr: self.id.clone(),
                    message_payload: Messages::Txs { Txs: txs },
                },
                id.clone(),
                calculate_delay(self.id.clone(), id.clone()),
            );
        }
    }

    fn req_tx_handler(&self, request_from: Id) {
        let result = Vec::new();
        let n = self.mempool.len();
        for i in 0..min(n, 500) {
            result.push(self.mempool[n - i - 1]);
        }

        self.ctx.borrow_mut().emit(
            Message{
                sender_addr: self.id.clone(),
                message_payload: Messages::Txs { Txs: result },
            },
            request_from.clone(),
            calculate_delay(self.id.clone(), request_from.clone()),
        );
    }

    fn get_addr(&self, request_from: Id) {
        let result = Vec::new();
        self.ctx.borrow_mut().emit(
            Message{
                sender_addr: self.id.clone(),
                message_payload: Messages::Addrs { addrs: Vec::from_iter(self.peers.values().cloned()), },
            },
            request_from.clone(),
            calculate_delay(self.id.clone(), request_from.clone()),
        );
    }

    fn get_addr_resp(&self, request_from: Id, addrs: Vec<Id>) {
        for addr in addrs {
            self.known_hosts.insert(addr);
        }
    }

    fn get_block(&self, request_from: Id, last_known_height: u32) {}

    fn get_block_resp(&self, request_from: Id) {}

    fn custom(&self, request_from: Id) {}

    fn middlerware(&self, msg: Message) {}

}