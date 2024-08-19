use tokio::process::Command;

use crate::app::AppContext;

use super::FlowError;

pub async fn get_pfx(app: &AppContext, email: &str, password: &str) -> Result<Vec<u8>, FlowError> {
    let easy_rsa_command = app.get_easy_rsa_command();

    let private_key_file = app.get_client_cert_private_key_file(email);
    let client_cert_file = app.get_client_cert_file(email);

    let pfx_file = app.get_client_cert_pfx_file(email);

    let result = Command::new(easy_rsa_command.as_str())
        .arg("pkcs12")
        .arg("-inkey")
        .arg(private_key_file.as_str())
        .arg("-in")
        .arg(client_cert_file.as_str())
        .arg("-export")
        .arg("-out")
        .arg(pfx_file.as_str())
        .arg("password")
        .arg(format!("pass:{}", password))
        .output()
        .await
        .unwrap();

    FlowError::check_error(&result)?;

    let content = tokio::fs::read(pfx_file.as_str()).await.unwrap();

    Ok(content)
    /*
       let ca_data_path = app.settings.get_config_path().into_ca_data_path(ca_cn);

       let ca_cert_file_name = ca_data_path.to_ca_cert_file_name();

       //let ca = client_cert_path.into_ca_data_path(ca_cn);

       let client_cert_path = ca_data_path.into_client_cert_path(email);

       let pfx_file_name = client_cert_path.to_pfx_file_name();

       println!("PFX file name: '{}'", pfx_file_name.as_str());

       println!(
           "Private Key File: '{}'",
           client_cert_path.to_private_key_file_name().as_str()
       );

       println!(
           "Cert File: '{}'",
           client_cert_path.to_cert_file_name().as_str()
       );

       println!("CA Cert File: '{}'", ca_cert_file_name.as_str());

       let output = Command::new("openssl")
           .arg("pkcs12")
           .arg("-export")
           .arg("-legacy")
           .arg("-out")
           .arg(pfx_file_name.as_str())
           .arg("-inkey")
           .arg(client_cert_path.to_private_key_file_name().as_str())
           .arg("-in")
           .arg(client_cert_path.to_cert_file_name().as_str())
           .arg("-certfile")
           .arg(ca_cert_file_name.as_str())
           .arg("-passout")
           .arg(format!("pass:{}", password))
           .output()
           .await
           .unwrap();

       println!("Output: {:?}", output);

       let content = tokio::fs::read(pfx_file_name.as_str()).await.unwrap();

       Ok(content)
    */
    /*
    let private_key = crate::storage::cert::load_pem_private_key(app, ca_cn, email).await;

    let private_key = private_key.into_private_key().private_key_to_der().unwrap();

    let private_key: PKey<Private> = PKey::private_key_from_der(&private_key).unwrap();

    let certificate = crate::storage::cert::load_pem_certificate(app, ca_cn, email).await;

    let certificate = certificate.into_certificate();

    let ca = crate::storage::ca::load_certificate(app, ca_cn).await;

    let x509 = X509::from_pem(ca.as_slice()).unwrap();
    let mut ca: Stack<X509> = Stack::new().unwrap();
    ca.push(x509).unwrap();

    println!("Password: '{}'", password);


    let pfx = openssl::pkcs12::Pkcs12::builder()
        .name(email)
        .cert(&certificate)
        .ca(ca)
        .pkey(&private_key)
        .build2(password)
        .unwrap();

    Ok(pfx.to_der().unwrap())
         */
}
