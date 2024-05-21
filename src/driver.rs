use dslab::{Id, SimulationContext, Simulation};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

use crate::_structs_2::Message;
use crate::_messages_2::Messages;
use crate::common::{gen_tx, calculate_delay};

pub struct Driver {
    id: Id,
    ctx: Rc<RefCell<SimulationContext>>,

    rng: ThreadRng,

    node_ids: Vec<Id>,
}

impl Driver {
    pub fn new(ctx: SimulationContext) -> Self {
        Self {
            id: ctx.id(),
            ctx: Rc::new(RefCell::new(ctx)),
            node_ids: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn add_id(&mut self, ids: Vec<Id>) {
        self.node_ids = ids
    }

    pub fn run_scenario(&mut self, sim: &mut Simulation) {
        sim.step_for_duration(5.0);
        self.send_transactions(30, 10);
    }

    fn send_transactions(&mut self, n_tx: u32, n_nodes: usize) {
        let mut txs = Vec::new();
        for _i in 0..=n_tx {
            txs.push(gen_tx);
        }

        let mut nodes : Vec<_> = self.node_ids.choose_multiple(&mut self.rng, n_nodes).collect();
        for node in nodes {
            self.ctx.borrow_mut().emit(Message{
                sender_addr: self.id,
                message_payload: Messages::Txs{Txs: txs.clone()},
            },
            node.clone(),
            calculate_delay(self.id, node.clone()),
            );
        }
    }
}

// #[derive(Serialize, Clone)]
// pub struct ControlMessage {
//     pub message_type: ControlMessageType,
// }

// #[derive(Serialize, Clone)]
// pub enum ControlMessageType {
//     ConnectNodes { node_ids: Vec<Id> },
//     PromoteNodes {},
//     PromoteObjects {},
//     ProcessTransaction { tx: Tx },
// }
