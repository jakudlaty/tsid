use std::env;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rand::{RngCore, thread_rng};
use crate::{TSID, TsidFactory};


lazy_static! {
    static ref SHARED_FACTORY: Mutex<TsidFactory> = Mutex::new(read_env_and_create_factory());
}

const DEFAULT_NODE_BITS: u8 = 8;

fn read_env_and_create_factory() -> TsidFactory {
    let node_bits_env =  env::var("TSID_NODE_BITS");
    let node_id_env =  env::var("TSID_NODE_ID");

    let node_bits = match node_bits_env {
        Ok(val) => {
            val.parse().unwrap_or(DEFAULT_NODE_BITS)
        }
        Err(_) => 8u8
    };

    let random_node_val = thread_rng().next_u32();
    let node_id = match node_id_env {
        Ok(val) => {
            val.parse().unwrap_or(random_node_val)
        }
        Err(_) => random_node_val
    };

    TsidFactory::with_node_bits(node_bits, node_id)
}

pub fn create_tsid() -> TSID {
    let mut guard = SHARED_FACTORY.lock().unwrap();
    return guard.create()
}