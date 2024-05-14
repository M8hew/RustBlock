use dslab::{Id, SimulationContext};
use rand::{rngs::ThreadRng, Rng};
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;
use uuid::Uuid;

use crate::components::Tx;

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
            node_ids: vec![],
            rng: rand::thread_rng(),
        }
    }

    pub fn add_id(&mut self, id: Id) {
        self.node_ids.push(id)
    }

    pub fn run_trigger(&mut self) {
        // scenario
        /*

        1---3--4--5--7
         \ /      \ /
          2        6

        */
        self.ctx.borrow().emit(
            ControlMessage {
                message_type: ControlMessageType::ConnectNodes {
                    node_ids: vec![self.node_ids[0], self.node_ids[1], self.node_ids[3]],
                },
            },
            self.node_ids[2],
            0.0,
        );
        self.ctx.borrow().emit(
            ControlMessage {
                message_type: ControlMessageType::ConnectNodes {
                    node_ids: vec![self.node_ids[3], self.node_ids[5], self.node_ids[6]],
                },
            },
            self.node_ids[4],
            0.0,
        );

        for i in 1..30 {
            let node_send_to = self.rng.gen_range(0..=self.node_ids.len() - 1);
            let rand_delay: f64 = self.rng.gen();
            self.ctx.borrow().emit(
                ControlMessage {
                    message_type: ControlMessageType::ProcessTransaction {
                        tx: Tx {
                            tx_id: Uuid::new_v4().as_u128(),
                        },
                    },
                },
                self.node_ids[node_send_to],
                1.3 + rand_delay,
            );

            if i % 5 == 0 {
                self.ctx.borrow().emit(
                    ControlMessage {
                        message_type: ControlMessageType::PromoteObjects {},
                    },
                    self.node_ids[node_send_to],
                    1.31 + rand_delay,
                );
            }
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ControlMessage {
    pub message_type: ControlMessageType,
}

#[derive(Serialize, Clone)]
pub enum ControlMessageType {
    ConnectNodes { node_ids: Vec<Id> },
    PromoteNodes {},
    PromoteObjects {},
    ProcessTransaction { tx: Tx },
}
