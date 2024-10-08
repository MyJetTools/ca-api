use crate::storage::cert::ClientCertPath;

const CA_CERT_FILE_NAME: &str = "ca_cert.pem";

const CA_PRIVATE_KEY_FILE_NAME: &str = "ca_private_key.pem";

const SERIAL_FILE_NAME: &str = "serial";

const CRL_FILE_NAME: &str = "crl.pem";
const INDEX_FILE_NAME: &str = "index.txt";
const INDEX_ATTR_FILE_NAME: &str = "index.txt.attr";

const CONFIG_FILE_NAME: &str = "openssl.cnf";

#[derive(Clone)]
pub struct CaDataPath {
    path: String,
}

impl CaDataPath {
    pub fn new(mut path: String, ca_cn: &str) -> Self {
        if !path.ends_with(std::path::MAIN_SEPARATOR) {
            path.push(std::path::MAIN_SEPARATOR);
        }
        path.push_str(ca_cn);
        Self { path }
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }

    fn into_file_name(self, file_name: &str) -> String {
        let mut result = self.path;

        if !result.ends_with('/') {
            result.push('/');
        }

        result.push_str(file_name);

        result
    }

    pub fn into_client_cert_path(self, email: &str) -> ClientCertPath {
        ClientCertPath::from_ca_path(self, email)
    }

    pub fn into_cert_file_name(self) -> String {
        self.into_file_name(CA_CERT_FILE_NAME)
    }

    pub fn to_ca_cert_file_name(&self) -> String {
        let result: CaDataPath = self.clone();
        result.into_file_name(CA_CERT_FILE_NAME)
    }

    pub fn into_private_key_file_name(self) -> String {
        self.into_file_name(CA_PRIVATE_KEY_FILE_NAME)
    }

    pub fn to_ca_private_key_file_name(&self) -> String {
        let result = self.clone();
        result.into_file_name(CA_PRIVATE_KEY_FILE_NAME)
    }

    pub fn into_serial_file_name(self) -> String {
        self.into_file_name(SERIAL_FILE_NAME)
    }

    pub fn to_serial_file_name(&self) -> String {
        let result = self.clone();
        result.into_file_name(SERIAL_FILE_NAME)
    }

    pub fn to_pfx_file_name(&self) -> String {
        let result = self.clone();
        result.into_file_name("ca.pfx")
    }
    /*
       pub fn into_index_file_name(self) -> String {
           self.into_file_name(INDEX_FILE_NAME)
       }
    */
    pub fn to_index_file_name(&self) -> String {
        let result = self.clone();
        result.into_file_name(INDEX_FILE_NAME)
    }

    /*
       pub fn into_index_attr_file_name(self) -> String {
           self.into_file_name(INDEX_ATTR_FILE_NAME)
       }
    */
    pub fn to_index_attr_file_name(&self) -> String {
        let result = self.clone();
        result.into_file_name(INDEX_ATTR_FILE_NAME)
    }

    /*
       pub fn into_config_file_name(self) -> String {
           self.into_file_name(CONFIG_FILE_NAME)
       }
    */
    pub fn to_config_file_name(&self) -> String {
        let result = self.clone();
        result.into_file_name(CONFIG_FILE_NAME)
    }

    pub fn to_ca_certs_path(&self) -> String {
        let result = self.clone();
        result.into_file_name("certs")
    }

    pub fn into_crl_file_name(self) -> String {
        self.into_file_name(CRL_FILE_NAME)
    }

    pub fn to_crl_file_name(&self) -> String {
        let result = self.clone();
        result.into_file_name(CRL_FILE_NAME)
    }

    /*
    pub fn get_ca_cert_file(&self, ca_cn: &str) -> String {
        let mut result = self.path.clone();
        result.push_str("ca_");
        result.push_str(ca_cn);
        result.push_str(".crt");
        result
    }
     */

    /*
       pub fn into_public_key_file_name(self) -> String {
           self.into_file_name(CA_PUBLIC_KEY_FILE_NAME)
       }
    */
}

impl Into<String> for CaDataPath {
    fn into(self) -> String {
        self.path
    }
}
