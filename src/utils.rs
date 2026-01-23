use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use prost_reflect::{DescriptorPool, DynamicMessage};
use std::error::Error;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

/// Байты → HEX
pub fn to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn load_proto_from_base64(base64_proto: &str) -> Result<DescriptorPool, Box<dyn Error>> {
    // 1. decode base64 -> .proto text
    let proto_bytes = STANDARD.decode(base64_proto)?;
    let proto_text = String::from_utf8(proto_bytes)?;

    // 2. temp dir
    let dir = tempdir()?;
    let proto_path = dir.path().join("schema.proto");
    let desc_path = dir.path().join("schema.desc");

    // 3. write proto file
    fs::write(&proto_path, proto_text)?;

    // 4. call protoc
    let output = Command::new("protoc")
        .arg(format!("--proto_path={}", dir.path().display()))
        .arg(format!("--descriptor_set_out={}", desc_path.display()))
        .arg("--include_imports")
        .arg(proto_path.file_name().unwrap())
        .current_dir(dir.path())
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "protoc failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // 5. load descriptor set
    let desc_bytes = fs::read(desc_path)?;
    let pool = DescriptorPool::decode(desc_bytes.as_slice())?;

    Ok(pool)
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
