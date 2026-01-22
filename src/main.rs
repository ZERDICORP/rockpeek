mod db;
mod utils;
mod cli;

use clap::Parser;
use cli::Cli;
use db::RockPeekDB;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Разбираем аргументы CLI
    let args = Cli::parse();

    // Открываем базу (только путь)
    let rockpeek = RockPeekDB::open_read_only(&args.db_path)?;

    // Сканируем указанную column family
    rockpeek.scan(&args.column_family)?;

    Ok(())
}

