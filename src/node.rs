// use dslab::{cast, Event, EventHandler, Id, SimulationContext};

// use crate::components::{ObjHash, ObjType, Services, Tx};
// use crate::driver::{ControlMessage, ControlMessageType};
// use crate::messages::{NetworkMessage, NetworkMessageType};
// use crate::tmp::rand_time;

// use std::cell::RefCell;
// use std::collections::{HashMap, HashSet};
// use std::rc::Rc;
// use std::{fmt, vec};

// pub struct BitcoinNode {
//     id: Id,

//     est_connections: HashSet<Id>,
//     // // metainfo
//     // mining_power: u32,
//     peers: HashMap<Id, f64>,  // peer addresses
//     connections: HashSet<Id>, // active connections

//     known_objects: HashMap<ObjHash, ObjType>,

//     mempool: Vec<Tx>,
//     ctx: Rc<RefCell<SimulationContext>>,
// }

// // imp trait with blanket implementations
// impl BitcoinNode {
//     pub fn new(ctx: SimulationContext) -> Self {
//         Self {
//             id: ctx.id(),

//             est_connections: HashSet::new(),

//             peers: HashMap::new(),
//             connections: HashSet::new(),

//             known_objects: HashMap::new(),
//             mempool: Vec::new(),

//             ctx: Rc::new(RefCell::new(ctx)),
//         }
//     }

//     pub fn handle_control_message(&mut self, message: ControlMessageType) {
//         match message {
//             ControlMessageType::ConnectNodes { node_ids } => {
//                 println!(
//                     "{:0.3}\t node {}:\t got ContolMessage::ConnectNodes",
//                     self.ctx.borrow().time(),
//                     self.id
//                 );

//                 for id in node_ids {
//                     self.connect(id)
//                 }
//             }
//             ControlMessageType::PromoteNodes {} => {
//                 println!(
//                     "{:0.3}\t node {}:\t got ContolMessage::PromoteNodes",
//                     self.ctx.borrow().time(),
//                     self.id
//                 );
//                 self.promote_nodes()
//             }
//             ControlMessageType::PromoteObjects {} => {
//                 println!(
//                     "{:0.3}\t node {}:\t got ContolMessage::PromoteObjects",
//                     self.ctx.borrow().time(),
//                     self.id
//                 );
//                 self.promote_objects()
//             }
//             ControlMessageType::ProcessTransaction { tx } => {
//                 println!(
//                     "{:0.3}\t node {}:\t got ContolMessage::ProcessTransaction",
//                     self.ctx.borrow().time(),
//                     self.id
//                 );
//                 self.known_objects.insert(tx.tx_id, ObjType::Tx);
//                 self.mempool.push(tx);
//             }
//         }
//     }

//     pub fn handle_network_message(&mut self, message: NetworkMessageType) {
//         match message {
//             NetworkMessageType::Version {
//                 version: _,
//                 services: _,
//                 timestamp: _,
//                 addr_from,
//             } => {
//                 println!(
//                     "{:0.3}\t node {}:\t got NetworkMessage::Version",
//                     self.ctx.borrow().time(),
//                     self.id
//                 );
//                 if self.est_connections.contains(&addr_from) {
//                     self.est_connections.remove(&addr_from);
//                     self.connections.insert(addr_from);
//                 } else {
//                     self.connections.insert(addr_from);
//                     let msg = NetworkMessageType::Version {
//                         version: 70001,
//                         services: Services::NodeNetwork,
//                         timestamp: self.ctx.borrow().time(),
//                         addr_from: self.id,
//                     };

//                     self.ctx.borrow_mut().emit(
//                         NetworkMessage { message_type: msg },
//                         addr_from,
//                         rand_time(0.1, 0.5),
//                     );

//                     println!(
//                         "{:0.3}\t node {}:\t send NetworkMessage::Version",
//                         self.ctx.borrow().time(),
//                         self.id
//                     );
//                 }
//             }
//             NetworkMessageType::Addr { addr_list } => {
//                 println!(
//                     "{:0.3}\t node {}:\t got NetworkMessage::Addr",
//                     self.ctx.borrow().time(),
//                     self.id
//                 );

//                 for (ts, id) in addr_list {
//                     self.peers.insert(id, ts);
//                 }
//             }
//             NetworkMessageType::Inv {
//                 inventory,
//                 addr_from,
//             } => {
//                 println!(
//                     "{:0.3}\t node {}:\t got NetworkMessage::Inv",
//                     self.ctx.borrow().time(),
//                     self.id
//                 );

//                 let mut new_objs = vec![];
//                 for (type_, hash_) in inventory {
//                     if !self.known_objects.contains_key(&hash_) {
//                         new_objs.push((type_.clone(), hash_));
//                         // move this logic out to getdata response
//                         self.known_objects.insert(hash_, type_);
//                     }
//                 }

//                 self.ctx.borrow().emit(
//                     NetworkMessage {
//                         message_type: NetworkMessageType::GetData {
//                             inventory: new_objs,
//                             addr_from: self.id,
//                         },
//                     },
//                     addr_from,
//                     rand_time(0.1, 0.3),
//                 );
//             }
//             NetworkMessageType::GetData {
//                 inventory,
//                 addr_from,
//             } => {
//                 println!(
//                     "{:0.3}\t node {}:\t got NetworkMessage::GetData",
//                     self.ctx.borrow().time(),
//                     self.id
//                 );

//                 let mut unknown_objs = vec![];
//                 let mut known_asked_objs = vec![];
//                 for (type_, hash_) in inventory {
//                     if !self.known_objects.contains_key(&hash_) {
//                         unknown_objs.push((type_, hash_));
//                     } else {
//                         known_asked_objs.push((type_, hash_));
//                     }
//                 }

//                 if unknown_objs.len() != 0 {
//                     self.ctx.borrow().emit(
//                         NetworkMessage {
//                             message_type: NetworkMessageType::NotFound {
//                                 inventory: unknown_objs,
//                             },
//                         },
//                         addr_from.clone(),
//                         rand_time(0.1, 0.3),
//                     );
//                     return;
//                 }

//                 for (type_, hash_) in known_asked_objs {
//                     match type_ {
//                         ObjType::Tx => {
//                             self.ctx.borrow().emit(
//                                 NetworkMessage {
//                                     message_type: NetworkMessageType::Tx {
//                                         tx: Tx { tx_id: hash_ },
//                                     },
//                                 },
//                                 addr_from.clone(),
//                                 rand_time(0.1, 0.3),
//                             );
//                         }
//                         ObjType::Block => {}
//                     }
//                 }
//                 // return all asked objects
//             }
//             NetworkMessageType::NotFound { inventory: _ } => {
//                 println!(
//                     "{:0.3}\t node {}:\t got NetworkMessage::NotFound",
//                     self.ctx.borrow().time(),
//                     self.id
//                 );
//                 // Repeat request with objets u interestred in
//             }
//             NetworkMessageType::Tx { tx: tx_ } => {
//                 println!(
//                     "{:0.3}\t node {}:\t got NetworkMessage::Tx",
//                     self.ctx.borrow().time(),
//                     self.id
//                 );

//                 if !self.known_objects.contains_key(&tx_.tx_id) {
//                     self.known_objects.insert(tx_.tx_id, ObjType::Tx);
//                 }
//             }
//         }
//     }

//     pub fn connect(&mut self, node_id: Id) {
//         let msg = NetworkMessageType::Version {
//             version: 70001,
//             services: Services::NodeNetwork,
//             timestamp: self.ctx.borrow().time(),
//             addr_from: self.id,
//         };

//         self.est_connections.insert(node_id);

//         self.ctx.borrow_mut().emit(
//             NetworkMessage { message_type: msg },
//             node_id,
//             rand_time(0.1, 0.5),
//         );

//         println!(
//             "{:0.3}\t node {}:\t send NetworkMessage::Version",
//             self.ctx.borrow().time(),
//             self.id
//         );
//     }

//     pub fn promote_nodes(&mut self) {
//         let mut addr_list = vec![];
//         for id in self.connections.iter() {
//             addr_list.push((self.ctx.borrow().time(), id.clone()))
//         }

//         for (id, ts) in self.peers.iter() {
//             addr_list.push((*ts, *id));
//         }

//         for id in self.connections.iter() {
//             self.ctx.borrow_mut().emit(
//                 NetworkMessage {
//                     message_type: NetworkMessageType::Addr {
//                         addr_list: addr_list.clone(),
//                     },
//                 },
//                 id.clone(),
//                 rand_time(0.1, 0.3),
//             );
//         }
//     }

//     pub fn promote_objects(&mut self) {
//         let mut objs = vec![];
//         for (hash_, type_) in self.known_objects.iter() {
//             objs.push((type_.clone(), *hash_));
//         }

//         for id in self.connections.iter() {
//             self.ctx.borrow().emit(
//                 NetworkMessage {
//                     message_type: NetworkMessageType::Inv {
//                         inventory: objs.clone(),
//                         addr_from: self.id,
//                     },
//                 },
//                 *id,
//                 rand_time(0.1, 0.3),
//             );
//         }
//     }
// }

// impl EventHandler for BitcoinNode {
//     fn on(&mut self, event: Event) {
//         cast! {match event.data {
//             ControlMessage{message_type} => {
//                 self.handle_control_message(message_type)
//             }
//             NetworkMessage{message_type} => {
//                 self.handle_network_message(message_type)
//             }
//         }}
//     }
// }

// impl fmt::Debug for BitcoinNode {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         fmt.debug_map()
//             .entry(&"node ", &self.id)
//             .entry(&"est_connections:", &self.est_connections)
//             .entry(&"peers", &self.peers)
//             .entry(&"connections", &self.connections)
//             .entry(&"known_objects", &self.known_objects)
//             .finish()
//     }
// }
