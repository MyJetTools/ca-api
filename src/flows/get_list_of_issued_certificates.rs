use crate::app::AppContext;

pub struct IssuedCertificateInfo {
    pub cn: String,
    pub revoked: bool,
}

pub async fn get_list_of_issued_certificates(app: &AppContext) -> Vec<IssuedCertificateInfo> {
    let temp_dir = app.settings.get_temp_dir();

    let mut dir_entry = tokio::fs::read_dir(temp_dir.as_str()).await.unwrap();

    let mut result = Vec::new();

    while let Some(entry) = dir_entry.next_entry().await.unwrap() {
        if entry
            .file_name()
            .as_os_str()
            .to_string_lossy()
            .ends_with(".p12")
        {
            result.push(IssuedCertificateInfo {
                cn: entry.file_name().as_os_str().to_string_lossy().to_string(),
                revoked: false,
            });
        }
    }

    result
}
