use serde::Serialize;

use crate::components::{PeerInfo, Tx};
use dslab::Id;

#[derive(Serialize, Clone)]
pub enum Messages {
    Connect {
        info : PeerInfo,
    },
    Txs {
        Txs: Vec<Tx>,
    },
    TxReq {
    },
    Addrs {
        addrs: Vec<Id>,
    },
    AddrReq{
    },
    BlockReq{        
    },
}