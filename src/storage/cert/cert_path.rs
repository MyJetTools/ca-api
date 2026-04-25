use crate::storage::ca::CaDataPath;

const CERT_FILE_NAME: &str = "cert.pem";
const PRIVATE_KEY_FILE_NAME: &str = "private_key.pem";

#[derive(Clone)]
pub struct ClientCertPath {
    path: String,
}

impl ClientCertPath {
    pub fn from_ca_path(path: CaDataPath, email: &str) -> Self {
        let mut path: String = path.into();
        let sub_path = email.replace("@", "_");

        path.push_str("/certs/");
        path.push_str(sub_path.as_str());

        Self { path }
    }

    fn into_file_name(self, file_name: &str) -> String {
        let mut result = self.path;
        if !result.ends_with('/') {
            result.push('/');
        }
        result.push_str(file_name);
        result
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }

    pub fn to_cert_file_name(&self) -> String {
        self.clone().into_file_name(CERT_FILE_NAME)
    }

    pub fn to_private_key_file_name(&self) -> String {
        self.clone().into_file_name(PRIVATE_KEY_FILE_NAME)
    }
}
