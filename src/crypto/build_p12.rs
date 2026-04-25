use openssl::pkcs12::Pkcs12;
use openssl::pkey::PKey;
use openssl::stack::Stack;
use openssl::x509::X509;

pub fn build_p12(
    cert_pem: &[u8],
    key_pem: &[u8],
    ca_pem: &[u8],
    name: &str,
    password: &str,
) -> Result<Vec<u8>, String> {
    let cert = X509::from_pem(cert_pem).map_err(|e| e.to_string())?;
    let key = PKey::private_key_from_pem(key_pem).map_err(|e| e.to_string())?;
    let ca_cert = X509::from_pem(ca_pem).map_err(|e| e.to_string())?;

    let mut chain = Stack::new().map_err(|e| e.to_string())?;
    chain.push(ca_cert).map_err(|e| e.to_string())?;

    let pkcs12 = Pkcs12::builder()
        .name(name)
        .pkey(&key)
        .cert(&cert)
        .ca(chain)
        .build2(password)
        .map_err(|e| e.to_string())?;

    pkcs12.to_der().map_err(|e| e.to_string())
}
