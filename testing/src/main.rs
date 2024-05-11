// TODO: Solve why this import is broken

// use sp_runtime::traits::AtLeast32Bit;

// fn main() {
//     let chain_id: u32 = 31337;
//     let chain_type: [u8; 2] = [2, 0];
//     println!("outcome: {}", compute_chain_id_type(chain_id, chain_type));
// }

// pub fn compute_chain_id_type<ChainId>(chain_id: ChainId, chain_type: [u8; 2]) -> u64
// where
//     ChainId: AtLeast32Bit,
// {
//     let mut chain_id_value: u32 = chain_id.try_into().unwrap_or_default();
//     println!("chainIdVal: {}", chain_id_value);
//     let mut buf = [0u8; 8];
//     println!("buf1: {:?}", buf);
//     buf[2..4].copy_from_slice(&chain_type);
//     println!("buf2: {:?}", buf);
//     println!(
//         "chainIdVal as byte array: {:?}",
//         chain_id_value.to_be_bytes()
//     );
//     buf[4..8].copy_from_slice(&chain_id_value.to_be_bytes());
//     println!("buf3: {:?}", buf);
//     u64::from_be_bytes(buf)
// }

// let hours: i32 = -12;
// let a = { hours % 24 }.abs();
// let b = (hours % 24).abs();

// let slice = vec!['r', 'u', 's', 't'];
// let all: () = slice
//     .windows(2)
//     .enumerate()
//     .map(|(i, win)| {
//         println!("{:?}", win);
//     })
//     .collect();
