use rocksdb::{DB, Options, ColumnFamilyDescriptor, IteratorMode};
use crate::utils::to_hex;
use std::error::Error;

/// Структура для работы с RocksDB
pub struct RockPeekDB {
    db: DB,
}

impl RockPeekDB {
    /// Read-only открытие базы с поддержкой всех CF
    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        // Получаем список всех CF в базе
        let cfs = DB::list_cf(&Options::default(), path)
            .unwrap_or_else(|_| vec!["default".to_string()]);

        // Создаём дескрипторы CF
        let cf_descriptors: Vec<ColumnFamilyDescriptor> = cfs
            .iter()
            .map(|name| ColumnFamilyDescriptor::new(name, Options::default()))
            .collect();

        // Открываем базу read-only с этими CF
        let db = DB::open_cf_descriptors_read_only(&Options::default(), path, cf_descriptors, false)?;
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

