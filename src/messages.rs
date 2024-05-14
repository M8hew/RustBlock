use dslab::Id;
use serde::Serialize;

use crate::components::{ObjHash, ObjType, Services, Tx};

#[derive(Serialize, Clone)]
pub struct NetworkMessage {
    pub message_type: NetworkMessageType,
}

#[derive(Serialize, Clone)]
pub enum NetworkMessageType {
    /// Node advertise its version when creates an outgoing connection.
    /// The remote node will respond with its version.
    /// No further communication is possible until both peers have exchanged their version
    Version {
        version: i32, // >70001
        services: Services,
        timestamp: f64,
        addr_from: Id,
        // nonce: u64,
        // start_height: i32,

        // relay: bool,
    },

    /// Provide information on known nodes of the network.
    /// Non-advertised nodes should be forgotten after typically 3 hours
    Addr {
        // 1000 max (timestamp + net_addr)
        addr_list: Vec<(f64, Id)>,
    },

    /// Allows a node to advertise its knowledge of one or more objects.
    /// It can be received unsolicited, or in reply to getblocks.
    Inv {
        // 50_000 max
        inventory: Vec<(ObjType, ObjHash)>,
        addr_from: Id,
    },

    /// Used in response to inv, to retrieve the content of a specific object
    /// or sent after receiving an inv packet, after filtering known elements.
    /// It can be used to retrieve transactions, but only if they are in the memory pool or relay set
    GetData {
        // 50_000 max
        inventory: Vec<(ObjType, ObjHash)>,
        addr_from: Id,
    },

    /// Response to a getdata, sent if any requested data items could not be relayed
    NotFound {
        // 50_000 max
        inventory: Vec<(ObjType, ObjHash)>,
    },

    /// Tx describes a bitcoin transaction, in reply to getdata
    Tx { tx: Tx },
}

/// Return an inv packet containing the list of blocks starting right after the last known hash
/// in the block locator object, up to hash_stop or 500 blocks, whichever comes first
// pub struct GetBlocks {
//     version: u32,
//     hash_count: u32,
//     block_locator_hashes: [u8; 32], // block locator
//     hash_stop: [u8; 32], // last desired block hash; or set zero to get max possible(500) blocks
// }

/// Return a headers packet containing the headers of blocks starting right after the last known
/// hash in the block locator object, up to hash_stop or 2000 blocks, whichever comes first
// pub struct GetHeaders {
//     version: u32,
//     hash_count: u32,
//     block_locator_hashes: [u8; 32],
//     hash_stop: [u8; 32], // if set zero gets max possible(2000) blocks
// }

/// Block describtes a bitcoin block structure
pub struct Block {}
