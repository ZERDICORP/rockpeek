use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use prost_reflect::{DescriptorPool, DynamicMessage};
use std::error::Error;

/// Байты → HEX
pub fn to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Загружает DescriptorPool из base64 proto
pub fn load_proto_descriptor(base64_proto: &str) -> Result<DescriptorPool, Box<dyn Error>> {
    let proto_bytes = STANDARD.decode(base64_proto)?;
    let pool = DescriptorPool::decode(proto_bytes.as_slice())?;
    Ok(pool)
}

/// Декодирует protobuf сообщение по имени
pub fn decode_message(
    pool: &DescriptorPool,
    message_name: &str,
    bytes: &[u8],
) -> Result<DynamicMessage, Box<dyn Error>> {
    let desc = pool
        .get_message_by_name(message_name)
        .ok_or_else(|| format!("Message '{}' not found in proto", message_name))?;

    let msg = DynamicMessage::decode(desc, bytes)?;
    Ok(msg)
}
