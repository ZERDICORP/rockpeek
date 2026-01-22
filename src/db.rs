use rocksdb::{DB, IteratorMode, Options};
use std::error::Error;

pub struct RockPeekDB {
    db: DB,
}

impl RockPeekDB {
    pub fn open_read_only(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(false);

        // Список имен всех CF
        let cf_names: Vec<String> = DB::list_cf(&opts, path)?;

        // Открываем DB в read-only режиме с этими CF
        let db = DB::open_cf_for_read_only(&opts, path, cf_names, false)?;

        Ok(Self { db })
    }

    pub fn iter_cf(
        &self,
        cf_name: &str,
    ) -> Result<
        impl Iterator<Item = Result<(Box<[u8]>, Box<[u8]>), rocksdb::Error>> + '_,
        Box<dyn Error>,
    > {
        let cf = self
            .db
            .cf_handle(cf_name)
            .ok_or_else(|| format!("Column family '{}' not found", cf_name))?;

        Ok(self.db.iterator_cf(cf, IteratorMode::Start))
    }
}
