use crate::storage::ca::CaDataPath;

pub struct ConfigPath {
    path: String,
}

impl ConfigPath {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }

    pub fn into_ca_data_path(self, ca_cn: &str) -> CaDataPath {
        CaDataPath::new(self.path, ca_cn)
    }
}
