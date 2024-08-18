use crate::{app::AppContext, pem::PemCertInfo};

pub async fn init(
    app: &AppContext,
    cert_info: &PemCertInfo,
    ca_cert_file_name: &str,
    ca_private_key_file_name: &str,
    config_file_name: &str,
    ca_certs_path: &str,
    //cert_ca: PemCertificate,
    //private_key: PemPrivateKey,
) {
    let ca_data_path = app
        .settings
        .get_config_path()
        .into_ca_data_path(&cert_info.ca_cn);

    println!("ca_path: '{}'", ca_data_path.as_str());

    tokio::fs::create_dir(ca_data_path.as_str()).await.unwrap();

    let serial_file_name = ca_data_path.to_serial_file_name();
    tokio::fs::write(serial_file_name.as_str(), "00")
        .await
        .unwrap();

    let index_file_name = ca_data_path.to_index_file_name();
    tokio::fs::write(index_file_name.as_str(), "")
        .await
        .unwrap();

    //let ca_cert_file_name = ca_data_path.to_cert_file_name();

    //   tokio::fs::write(ca_cert_file_name.as_str(), cert_ca.as_slice())
    //      .await
    //      .unwrap();

    //let ssl_path = SslCertsPath::new(&app.settings_reader).await;
    //let ca_path = app.settings.get_config_path().into_ca_data_path();

    /*
       tokio::fs::write(
           ca_data_path.get_ca_cert_file(&cert_info.ca_cn),
           cert_ca.as_slice(),
       )
       .await
       .unwrap();
    */

    //   tokio::fs::write(ca_cert_file_name.as_str(), cert_ca.as_slice())
    //       .await
    //       .unwrap();

    //let ca_private_key_file_name = ca_data_path.to_private_key_file_name();
    //tokio::fs::write(ca_private_key_file_name.as_str(), private_key.as_slice())
    //    .await
    //    .unwrap();

    tokio::fs::write(
        config_file_name,
        format!(
            r#"
[ ca ]
default_ca = {ca_cn}

[ {ca_cn} ]
dir = {path}
database = {index_file_name}
certificate = {ca_cert_file_name}
private_key = {ca_private_key_file_name}
new_certs_dir     =  {ca_certs_path}
serial = {serial_file_name}
default_days = 3650
default_md = sha256
policy = policy_any
default_crl_days = 3650

[ policy_any ]
countryName = optional
organizationName = optional
localityName = optional

[ req ]
default_bits        = 4096
default_keyfile     = {ca_private_key_file_name}
default_md          = sha256
prompt              = no
distinguished_name  = req_distinguished_name
x509_extensions     = v3_ca


[ req_distinguished_name ]
C  = {country_code}
ST = {city}
L  = {city}
O  = {organization_name}
OU = {organization_name}
CN = {ca_cn}


[ v3_ca ]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always,issuer
basicConstraints = critical, CA:true
keyUsage = critical, digitalSignature, cRLSign, keyCertSign

"#,
            path = ca_data_path.as_str(),
            ca_cn = cert_info.ca_cn,
            country_code = cert_info.country_code,
            organization_name = cert_info.organization,
            city = cert_info.city,
        ),
    )
    .await
    .unwrap();

    tokio::fs::write(
        ca_data_path.to_index_attr_file_name(),
        "unique_subject = yes",
    )
    .await
    .unwrap();
}
