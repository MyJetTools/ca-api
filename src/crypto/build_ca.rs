use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use rcgen::{
    BasicConstraints, CertificateParams, DistinguishedName, DnType, IsCa, KeyPair,
    KeyUsagePurpose,
};
use time::{Duration, OffsetDateTime};

use crate::pem::PemCertInfo;

pub struct GeneratedCa {
    pub cert_pem: String,
    pub key_pem: String,
}

pub fn build_ca(info: &PemCertInfo) -> Result<GeneratedCa, String> {
    let key_pem = generate_rsa_key_pem()?;
    let key_pair = KeyPair::from_pem(&key_pem).map_err(|e| e.to_string())?;

    let mut params = CertificateParams::default();

    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, &info.ca_cn);
    if !info.country_code.is_empty() {
        dn.push(DnType::CountryName, &info.country_code);
    }
    if !info.city.is_empty() {
        dn.push(DnType::LocalityName, &info.city);
        dn.push(DnType::StateOrProvinceName, &info.city);
    }
    if !info.organization.is_empty() {
        dn.push(DnType::OrganizationName, &info.organization);
    }
    dn.push(DnType::OrganizationalUnitName, "IT");
    params.distinguished_name = dn;

    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.key_usages = vec![
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::CrlSign,
    ];

    let now = OffsetDateTime::now_utc();
    params.not_before = now;
    params.not_after = now + Duration::days(365 * 10);

    let cert = params.self_signed(&key_pair).map_err(|e| e.to_string())?;

    Ok(GeneratedCa {
        cert_pem: cert.pem(),
        key_pem,
    })
}

pub(crate) fn generate_rsa_key_pem() -> Result<String, String> {
    let rsa = Rsa::generate(4096).map_err(|e| e.to_string())?;
    let pkey = PKey::from_rsa(rsa).map_err(|e| e.to_string())?;
    let pem = pkey
        .private_key_to_pem_pkcs8()
        .map_err(|e| e.to_string())?;
    String::from_utf8(pem).map_err(|e| e.to_string())
}
