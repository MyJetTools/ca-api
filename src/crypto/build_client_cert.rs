use rcgen::{
    CertificateParams, DistinguishedName, DnType, ExtendedKeyUsagePurpose, Issuer,
    KeyPair, KeyUsagePurpose, SerialNumber, SigningKey,
};
use time::{Duration, OffsetDateTime};

use super::generate_rsa_key_pem;

pub struct GeneratedClientCert {
    pub cert_pem: String,
    pub key_pem: String,
}

pub fn build_client_cert<S: SigningKey>(
    issuer: &Issuer<'_, S>,
    cn: &str,
    serial: u64,
) -> Result<GeneratedClientCert, String> {
    let key_pem = generate_rsa_key_pem()?;
    let key_pair = KeyPair::from_pem(&key_pem).map_err(|e| e.to_string())?;

    let mut params = CertificateParams::default();
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, cn);
    params.distinguished_name = dn;

    params.serial_number = Some(SerialNumber::from(serial));
    params.key_usages = vec![
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::KeyEncipherment,
    ];
    params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ClientAuth];

    let now = OffsetDateTime::now_utc();
    params.not_before = now;
    params.not_after = now + Duration::days(365 * 10);

    let cert = params
        .signed_by(&key_pair, issuer)
        .map_err(|e| e.to_string())?;

    Ok(GeneratedClientCert {
        cert_pem: cert.pem(),
        key_pem,
    })
}
