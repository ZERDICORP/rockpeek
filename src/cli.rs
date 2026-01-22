use clap::Parser;

/// RockPeek — утилита для сканирования RocksDB
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Путь к RocksDB
    #[arg(short = 'd', long = "db-path")]
    pub db_path: String,

    /// Название column family
    #[arg(short = 'c', long = "column-family")]
    pub column_family: String,
}

