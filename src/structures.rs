// /// Network adress of the node
// pub struct NetAddr {
//     time: u32,
//     services: u64, // bitmask
//     ip: [u8; 16],  // v4/v6
//     port: u16,
// }

// /// Type, hash pair specifying the object
// pub struct InvVect {
//     _type: u32,
//     hash: [u8; 32],
// }

// /// Placeholder for script_types
// pub struct Script {}

// // Output transaction reference
// pub struct OutPoint {
//     hash: [u8; 32],
//     index: u32,
// }

// // Transaction input
// pub struct TxIn {
//     prev_output: OutPoint, // previous output transaction reference
//     signature_script: Script,
//     sequence: u32,
// }

// // Transaction output
// pub struct TxOut {
//     value: i64,
//     pk_script: Script,
// }

// // Transation witness
// pub struct TxWitness {}

// /// Tx describes a bitcoin transaction
// pub struct Tx {
//     version: u32,
//     flag: std::option<u16>, // indicates the presence of withness data
//     tx_in_count: u32,
//     tx_in: [TxIn],
//     tx_out_count: u32,
//     tx_out_count: [TxOut],
//     tx_witnesses: [TxWitness],
//     lock_time: u32,
// }

// // Describes
// pub struct BlockHeader {}

// pub struct Block {}
