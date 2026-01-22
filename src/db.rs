use rocksdb::{IteratorMode, Options, DB};
use std::collections::HashSet;
use std::error::Error;
use std::path::Path;

pub struct RockPeekDB {
    db: DB,
}

impl RockPeekDB {
    /// Открываем RocksDB в read-only режиме
    pub fn open_read_only<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(false);

        let db = DB::open_for_read_only(&opts, path, false)?;
        Ok(Self { db })
    }

    /// Сканируем column family и вызываем callback на каждую запись
    pub fn scan_cf<F>(&self, cf_name: &str, mut on_record: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&[u8], &[u8]),
    {
        let cf = self
            .db
            .cf_handle(cf_name)
            .ok_or_else(|| format!("Column family '{}' not found", cf_name))?;

        let iter = self.db.iterator_cf(cf, IteratorMode::Start);

        let mut seen_keys: HashSet<Vec<u8>> = HashSet::new();

        for item in iter {
            let (key, value) = item?;

            // deduplication по ключу
            if !seen_keys.insert(key.to_vec()) {
                continue;
            }

            on_record(&key, &value);
        }

        Ok(())
    }
}
