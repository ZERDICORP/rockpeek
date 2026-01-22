use clap::Parser;

/// RockPeek CLI
#[derive(Parser)]
#[command(name = "rockpeek")]
pub struct Cli {
    /// Путь к RocksDB
    #[arg(short = 'd', long = "db-path")]
    pub db_path: String,

    /// Column Family
    #[arg(short = 'c', long = "column-family")]
    pub column_family: String,

    /// Файл .proto в base64
    #[arg(short = 'p', long = "proto")]
    pub proto: Option<String>,

    /// Proto Message ключа
    #[arg(short = 'k', long = "key-message")]
    pub key_message: Option<String>,

    /// Proto Message значения
    #[arg(short = 'v', long = "value-message")]
    pub value_message: Option<String>,
}
