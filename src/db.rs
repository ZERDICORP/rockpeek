use rocksdb::{DB, Options, IteratorMode};
use crate::utils::to_hex;
use std::error::Error;

/// Структура для работы с RocksDB
pub struct RockPeekDB {
    db: DB,
}

impl RockPeekDB {
    /// Открываем базу в режиме read-only
    pub fn open_read_only(path: &str) -> Result<Self, Box<dyn Error>> {
        let db = DB::open_for_read_only(&Options::default(), path, false)?;
        Ok(Self { db })
    }

    /// Сканируем указанную column family
    pub fn scan(&self, cf_name: &str) -> Result<(), Box<dyn Error>> {
        let cf_handle = self.db.cf_handle(cf_name)
            .ok_or(format!("Column family '{}' not found", cf_name))?;

        let iter = self.db.iterator_cf(cf_handle, IteratorMode::Start);

        for item in iter {
            match item {
                Ok((key, value)) => {
                    println!("0x{} ==> 0x{}", to_hex(&key), to_hex(&value));
                }
                Err(e) => eprintln!("Ошибка чтения ключа: {}", e),
            }
        }

        Ok(())
    }
}

