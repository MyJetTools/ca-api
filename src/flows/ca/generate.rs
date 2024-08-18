use std::sync::Arc;

use tokio::process::Command;

use crate::{app::AppContext, pem::PemCertInfo};

pub async fn generate(app: &Arc<AppContext>, cert_info: PemCertInfo) {
    init_vars(app, &cert_info).await;
    let easy_rsa_command = app.get_easy_rsa_command();

    let result = Command::new(easy_rsa_command.as_str())
        .arg("init-pki")
        .output()
        .await
        .unwrap();

    println!("Init PKI Output: {:?}", result);

    let result = Command::new(easy_rsa_command.as_str())
        .arg("build-ca")
        .arg("nopass")
        .output()
        .await
        .unwrap();

    println!("Build CA Output: {:?}", result);

    /*
    let ca_path = app
        .settings
        .get_config_path()
        .into_ca_data_path(&cert_info.ca_cn);

    let ca_private_key_file_name = ca_path.to_ca_private_key_file_name();

    let ca_cert_file_name = ca_path.to_ca_cert_file_name();

    let config_file_name = ca_path.to_config_file_name();

    let ca_certs_path = ca_path.to_ca_certs_path();

    crate::storage::ca::init(
        app,
        &cert_info,
        ca_cert_file_name.as_str(),
        ca_private_key_file_name.as_str(),
        config_file_name.as_str(),
        ca_certs_path.as_str(),
    )
    .await;

    println!("Generating CA Private Key");
    let output = Command::new("openssl")
        .arg("genpkey")
        .arg("-algorithm")
        .arg("RSA")
        .arg("-out")
        .arg(ca_private_key_file_name.as_str())
        .output()
        .await
        .unwrap();

    println!("Generating CA Private Key Output: {:?}", output);

    println!("Generating CA Root Certificate");
    let output = Command::new("openssl")
        .arg("req")
        .arg("-x509")
        .arg("-new")
        .arg("-key")
        .arg(ca_private_key_file_name.as_str())
        .arg("-sha256")
        .arg("-days")
        .arg("3650")
        .arg("-out")
        .arg(ca_cert_file_name.as_str())
        .arg("-config")
        .arg(config_file_name.as_str())
        .output()
        .await
        .unwrap();

    println!("Generating CA Private Key Output: {:?}", output);

     */
    // Generate a 2048 bit RSA private key for the CA
    /*
    let rsa_ca = Rsa::generate(4096).unwrap();
    let p_key_ca = PKey::from_rsa(rsa_ca).unwrap();

    // Build the X509 name for the CA
    let mut builder = X509NameBuilder::new().unwrap();
    builder
        .append_entry_by_nid(Nid::COMMONNAME, &cert_info.ca_cn)
        .unwrap();
    builder
        .append_entry_by_text("C", &cert_info.country_code)
        .unwrap();

    builder.append_entry_by_text("L", &cert_info.city).unwrap();
    builder
        .append_entry_by_text("O", &cert_info.organization)
        .unwrap();
    let name_ca = builder.build();

    // Build the CA certificate
    let mut cert_builder = X509::builder().unwrap();
    cert_builder.set_version(2).unwrap();
    let serial_number = openssl::bn::BigNum::from_u32(1)
        .unwrap()
        .to_asn1_integer()
        .unwrap();
    cert_builder.set_serial_number(&serial_number).unwrap();
    cert_builder.set_subject_name(&name_ca).unwrap();
    cert_builder.set_issuer_name(&name_ca).unwrap();
    let not_before = openssl::asn1::Asn1Time::days_from_now(0).unwrap();
    cert_builder.set_not_before(&not_before).unwrap();
    let not_after = openssl::asn1::Asn1Time::days_from_now(365).unwrap();
    cert_builder.set_not_after(&not_after).unwrap();
    cert_builder.set_pubkey(&p_key_ca).unwrap();

    cert_builder
        .append_extension(BasicConstraints::new().critical().ca().build().unwrap())
        .unwrap();

    cert_builder
        .sign(&p_key_ca, MessageDigest::sha256())
        .unwrap();
    let cert_ca = cert_builder.build();

    crate::storage::ca::write(app, &cert_info, cert_ca.into(), p_key_ca.into()).await
     */
}

async fn init_vars(app: &Arc<AppContext>, cert_info: &PemCertInfo) {
    let mut to_write = String::new();

    to_write.push_str("set_var EASYRSA_DN \"org\"\n");
    to_write.push_str(format!("set_var EASYRSA_REQ_CN \"{}\"\n", cert_info.ca_cn).as_str());
    to_write.push_str(
        format!(
            "set_var EASYRSA_REQ_COUNTRY \"{}\"\n",
            cert_info.country_code
        )
        .as_str(),
    );
    to_write.push_str(format!("set_var EASYRSA_REQ_PROVINCE \"{}\"\n", cert_info.city).as_str());
    to_write.push_str(format!("set_var EASYRSA_REQ_CITY \"{}\"\n", cert_info.city).as_str());
    to_write.push_str(format!("set_var EASYRSA_REQ_ORG \"{}\"\n", cert_info.organization).as_str());
    to_write.push_str(format!("set_var EASYRSA_REQ_EMAIL \"{}\"\n", cert_info.email).as_str());

    to_write.push_str("set_var EASYRSA_KEY_SIZE 4096\n");
    to_write.push_str("set_var EASYRSA_CA_EXPIRE 3650\n");
    to_write.push_str("set_var EASYRSA_CERT_EXPIRE 3650\n");

    let file_path = app.get_vars_path();

    tokio::fs::write(file_path, to_write.as_str())
        .await
        .unwrap();
}
