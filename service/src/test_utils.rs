use std::ops::Deref;

use abi::Config;
use db_sqlx_tester::TestDb;

pub struct TestConfig {
    #[allow(dead_code)]
    tdb: TestDb,
    config: Config,
}

impl TestConfig {
    pub fn new() -> Self {
        let mut config = Config::load("fixtures/config.yml").unwrap();
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
}

impl Deref for TestConfig {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}
