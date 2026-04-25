use rcgen::{
    CertificateRevocationListParams, Issuer, KeyIdMethod, RevokedCertParams, SerialNumber,
    SigningKey,
};
use time::{Duration, OffsetDateTime};

pub struct CrlEntry {
    pub serial: u64,
    pub revoked_at: OffsetDateTime,
}

pub fn build_crl<S: SigningKey>(
    issuer: &Issuer<'_, S>,
    entries: &[CrlEntry],
    crl_number: u64,
) -> Result<String, String> {
    let now = OffsetDateTime::now_utc();

    let revoked_certs = entries
        .iter()
        .map(|e| RevokedCertParams {
            serial_number: SerialNumber::from(e.serial),
            revocation_time: e.revoked_at,
            reason_code: None,
            invalidity_date: None,
        })
        .collect();

    let params = CertificateRevocationListParams {
        this_update: now,
        next_update: now + Duration::days(30),
        crl_number: SerialNumber::from(crl_number),
        issuing_distribution_point: None,
        revoked_certs,
        key_identifier_method: KeyIdMethod::Sha256,
    };

    let crl = params.signed_by(issuer).map_err(|e| e.to_string())?;
    crl.pem().map_err(|e| e.to_string())
}
