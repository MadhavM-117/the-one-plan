use envconfig::Envconfig;

#[derive(Clone, Envconfig)]
pub struct ServiceConfig {
    #[envconfig(from = "DATABASE_URL")]
    db_url: String,
    #[envconfig(from = "FILE_STORAGE_ROOT")]
    storage_root: String,
}

impl ServiceConfig {
    pub fn get_db_url(&self) -> &str {
        self.db_url.as_str()
    }

    pub fn get_storage_root(&self) -> &str {
        self.storage_root.as_str()
    }
}
