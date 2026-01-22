use rocksdb::{DB, Options, ColumnFamilyDescriptor, IteratorMode};
use crate::utils::to_hex;
use std::error::Error;

pub struct RockPeekDB {
    db: DB,
}

impl RockPeekDB {
    /// Открываем базу с перечислением всех column family (read-only)
    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        // Получаем список CF из базы
        let cfs = DB::list_cf(&Options::default(), path)
            .unwrap_or_else(|_| vec!["default".to_string()]);

        let cf_descriptors: Vec<ColumnFamilyDescriptor> = cfs
            .iter()
            .map(|name| ColumnFamilyDescriptor::new(name, Options::default()))
            .collect();

        // Открываем базу с CF дескрипторами
        let db = DB::open_cf_descriptors(&Options::default(), path, cf_descriptors)?;

        Ok(Self { db })
    }

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

