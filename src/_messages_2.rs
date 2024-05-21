use serde::Serialize;

use crate::_structs_2::{PeerInfo, Tx};

#[derive(Serialize, Clone)]
pub enum Messages {
    Connect {
        info : PeerInfo,
    },
    Txs {
        Txs: Vec<Tx>,
    }
}