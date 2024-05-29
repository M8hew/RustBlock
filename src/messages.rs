use serde::Serialize;

use crate::components::{PeerInfo, Tx};

#[derive(Serialize, Clone)]
pub enum Messages {
    Connect {
        info : PeerInfo,
    },
    Txs {
        Txs: Vec<Tx>,
    }
}