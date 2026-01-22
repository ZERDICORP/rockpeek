mod cli;
mod db;
mod utils;

use clap::Parser;
use cli::Cli;
use db::RockPeekDB;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // Загружаем proto, если он передан
    let proto_pool = match &args.proto {
        Some(p) => Some(utils::load_proto_descriptor(p)?),
        None => None,
    };

    let db = RockPeekDB::open_read_only(&args.db_path)?;

    db.scan_cf(&args.column_family, |key, value| {
        match (&proto_pool, &args.key_message, &args.value_message) {
            // protobuf режим
            (Some(pool), Some(key_msg), Some(value_msg)) => {
                let key_decoded = utils::decode_message(pool, key_msg, key);
                let value_decoded = utils::decode_message(pool, value_msg, value);

                match (key_decoded, value_decoded) {
                    (Ok(k), Ok(v)) => {
                        println!("{} ==> {}", k, v);
                    }
                    (Err(e), _) | (_, Err(e)) => {
                        eprintln!("protobuf decode error: {}", e);
                    }
                }
            }

            // HEX режим (дефолт)
            _ => {
                println!("0x{} ==> 0x{}", utils::to_hex(key), utils::to_hex(value));
            }
        }
    })?;

    Ok(())
}
