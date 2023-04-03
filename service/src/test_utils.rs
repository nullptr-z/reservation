use std::{ops::Deref, path::Path};

use abi::Config;
use db_sqlx_tester::TestDb;

pub struct TestConfig {
    #[allow(dead_code)]
    tdb: TestDb,
    pub config: Config,
}

impl TestConfig {
    pub fn new(filename: impl AsRef<Path>) -> Self {
        let mut config = Config::load(filename).unwrap();
        let tdb = TestDb::new(
            &config.db.host,
            config.db.port,
            &config.db.user,
            &config.db.password,
            "../migrations",
        );
        config.db.dbname = tdb.dbname.clone();
        Self { tdb, config }
    }

    pub fn with_server_port(prot: u16) -> Self {
        todo!();
        // let mut config = TestConfig::default();
        // config.config.ser
    }
}

impl Deref for TestConfig {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self::new("fixtures/config.yml")
    }
}
