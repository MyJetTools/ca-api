use std::sync::Arc;

use tokio::process::Command;

use crate::app::AppContext;

use super::FlowError;

pub async fn generate_cert(app: &Arc<AppContext>, email: &str) -> Result<(), FlowError> {
    let easy_rsa_command = app.get_easy_rsa_command();

    let result = Command::new(easy_rsa_command.as_str())
        .arg("build-client-full")
        .arg(email)
        .arg("nopass")
        .output()
        .await
        .unwrap();

    FlowError::check_error(&result)?;

    /*
       let ca_path = app.settings.get_config_path().into_ca_data_path(ca_cn);

       let config_file_name = ca_path.to_config_file_name();

       println!("Config: {}", config_file_name);

       let client_cert_path = ca_path.into_client_cert_path(email);

       let _ = tokio::fs::create_dir_all(client_cert_path.as_str()).await;

       let private_key_file_name = client_cert_path.to_private_key_file_name();
       let cert_request_file_name = client_cert_path.to_certificate_request_file_name();
       let client_cert_file_name = client_cert_path.to_cert_file_name();

       println!("Generating Client Certificate Private Key");
       let output = Command::new("openssl")
           .arg("genpkey")
           .arg("-algorithm")
           .arg("RSA")
           .arg("-out")
           .arg(private_key_file_name.as_str())
           .output()
           .await
           .unwrap();
       println!(
           "Generating Client Certificate Private Key Output: {:?}",
           output
       );

       println!("Generating Client Certificate CSR");
       let output = Command::new("openssl")
           .arg("req")
           .arg("-new")
           .arg("-key")
           .arg(private_key_file_name.as_str())
           .arg("-out")
           .arg(cert_request_file_name.as_str())
           .arg("-config")
           .arg(config_file_name.as_str())
           .output()
           .await
           .unwrap();
       println!("Generating Client Certificate CSR Output: {:?}", output);

       println!("Generating Client Certificate");
       let output = Command::new("openssl")
           .arg("ca")
           .arg("-config")
           .arg(config_file_name.as_str())
           .arg("-in")
           .arg(cert_request_file_name.as_str())
           .arg("-out")
           .arg(client_cert_file_name.as_str())
           .arg("-days")
           .arg("3650")
           .arg("-batch")
           .output()
           .await
           .unwrap();
       println!("Generating Client Certificate Output: {:?}", output);
    */
    /*
    println!("Loading private key");
    let ca_private_key = crate::storage::ca::load_private_key(app, ca_cn).await;

    //let ca_path = crate::storage::utils::get_ca_path(app, ca_cn).await;
    println!("Creating folder");
    let cert_path = crate::storage::cert::create_folder_if_not_exists(app, ca_cn, email).await;

    println!("Writing-1");
    // Generate a 2048 bit RSA private key for the client
    let rsa_client = Rsa::generate(4096).unwrap();
    let pkey_client = PKey::from_rsa(rsa_client).unwrap();

    tokio::fs::write(
        cert_path.to_private_key_file_name(),
        pkey_client.private_key_to_pem_pkcs8().unwrap(),
    )
    .await
    .unwrap();

    println!("Writing-2");
    tokio::fs::write(
        cert_path.to_public_key_file_name(),
        pkey_client.public_key_to_pem().unwrap(),
    )
    .await
    .unwrap();

    println!("Getting 509 name");
    let ca_name = crate::storage::ca::get_509_name(app, ca_cn).await;

    println!("Building X509 name");
    // Build the X509 name for the client
    let mut builder = X509NameBuilder::new().unwrap();
    builder.append_entry_by_nid(Nid::COMMONNAME, email).unwrap();
    let name_client = builder.build();

    let cert_serial_number = crate::storage::ca::get_next_serial_number(app, ca_cn).await;

    // Build the client certificate request
    let mut req_builder = X509ReqBuilder::new().unwrap();
    req_builder.set_version(2).unwrap();
    req_builder.set_subject_name(&name_client).unwrap();
    req_builder.set_pubkey(&pkey_client).unwrap();
    req_builder
        .sign(&pkey_client, MessageDigest::sha256())
        .unwrap();

    println!("Building client certificate-1");
    // Build the client certificate
    let mut cert_builder = X509::builder().unwrap();
    cert_builder.set_version(2).unwrap();
    let serial_number = openssl::bn::BigNum::from_u32(cert_serial_number)
        .unwrap()
        .to_asn1_integer()
        .unwrap();
    cert_builder.set_serial_number(&serial_number).unwrap();
    cert_builder.set_subject_name(&name_client).unwrap();
    cert_builder.set_issuer_name(&ca_name).unwrap();
    let not_before = openssl::asn1::Asn1Time::days_from_now(0).unwrap();
    cert_builder.set_not_before(&not_before).unwrap();
    let not_after = openssl::asn1::Asn1Time::days_from_now(365 * 10).unwrap();
    cert_builder.set_not_after(&not_after).unwrap();
    cert_builder.set_pubkey(&pkey_client).unwrap();
    println!("Building client certificate-2");
    cert_builder
        .sign(&ca_private_key.into_private_key(), MessageDigest::sha256())
        .unwrap();
    println!("Building client certificate-3");
    let cert_client = cert_builder.build();

    println!("Writing cert");
    tokio::fs::write(cert_path.to_cert_file_name(), cert_client.to_pem().unwrap())
        .await
        .unwrap();

         */
    Ok(())
}
