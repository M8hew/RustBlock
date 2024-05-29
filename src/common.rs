use std::rc::Rc;
use rand::Rng;
use rand::thread_rng;
use rand_distr::{Distribution, LogNormal, Exp};
use uuid::Uuid;
use twox_hash::xxh3::hash64;

use crate::components::{Tx, Block};

pub fn gen_peers(cur_id: u32, from: u32, to : u32, is_connected: fn(u32, u32)->bool) -> Vec<u32> {
    let mut result = Vec::new();
    for peer_id in from..=to {
        if is_connected(cur_id, peer_id) {
            result.push(peer_id);
        }
    }
    return result;
}

pub fn gen_tx() -> Tx {
    let uuid = Uuid::new_v4();
    let hash = hash64(uuid.as_bytes());

    let exp = Exp::new(1.5).unwrap();
    let mut rng = thread_rng();
    let fee = exp.sample(&mut rng);

    Tx {
        tx_id: hash, 
        tx_fee: fee, 
    }
}

pub fn create_chain() -> Rc<Block> {
    Rc::new(Block{
        block_id: 0,
        height: 0,
        prev_block: None,

        txs: Vec::new(),
    })
}

pub fn connect_with_uniform_prob(_first: u32, _second: u32) -> bool {
    let mut rng = rand::thread_rng();
    let uniform_sample: f64 = rng.gen_range(0.0..1.0);

    uniform_sample > 0.3
}

pub fn calculate_delay(_first_id: u32, _second_id: u32) -> f64 {
    let normal = LogNormal::new(1.0, 2.0).unwrap();
    let mut rng = rand::thread_rng();
    normal.sample(&mut rng)
}