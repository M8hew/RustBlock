use serde::Serialize;
use std::fmt;

// use std::rc::Rc;

pub type ObjHash = u128;

#[derive(Serialize, Clone)]
pub enum Services {
    NodeNetwork,
    NodeGetutxo,
    NodeBloom,
    NodeWitness,
    NodeXthin,
    NodeCompactFilters,
    NodeNetworkLimited,
}

#[derive(Serialize, Clone, Debug)]
pub enum ObjType {
    Block,
    Tx,
}

// pub struct Block {
//     version: u32,
//     prev_block: Rc<Block>,
//     block_body: Rc<BlockBody>,
// }

// pub struct BlockBody {
//     txs: Vec<Tx>,
// }

#[derive(Serialize, Clone)]
pub struct Tx {
    pub tx_id: ObjHash,
}

impl fmt::Debug for Tx {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Tx")
            .field("tx_id", &(self.tx_id % 1000))
            .finish()
    }
}
