use rocksdb::{DB, Options, ColumnFamilyDescriptor, IteratorMode};
use crate::utils::to_hex;
use std::error::Error;

/// Структура для работы с RocksDB
pub struct RockPeekDB {
    db: DB,
}

impl RockPeekDB {
    /// Открываем базу и все column family
    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        // Получаем список column family (если база пустая, дефолтная "default")
        let cfs = DB::list_cf(&Options::default(), path)
            .unwrap_or_else(|_| vec!["default".to_string()]);

        // Создаем дескрипторы column family
        let cf_descriptors: Vec<ColumnFamilyDescriptor> = cfs
            .into_iter()
            .map(|name| ColumnFamilyDescriptor::new(name, Options::default()))
            .collect();

        // Открываем базу с column family
        let db = DB::open_cf_descriptors(&Options::default(), path, cf_descriptors)?;

        Ok(Self { db })
    }

    /// Сканируем указанную column family и выводим ключи/значения в hex
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

