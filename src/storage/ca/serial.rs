use crate::storage::ca::CaDataPath;

pub async fn read_next_serial(ca_path: &CaDataPath) -> u64 {
    let file = ca_path.to_serial_file_name();

    let current = match tokio::fs::read_to_string(&file).await {
        Ok(text) => text.trim().parse::<u64>().unwrap_or(1),
        Err(_) => 1,
    };

    let next = current.checked_add(1).expect("Serial counter overflow");
    tokio::fs::write(&file, next.to_string()).await.unwrap();

    current
}

pub async fn read_crl_number(ca_path: &CaDataPath) -> u64 {
    let file = crl_number_file(ca_path);

    let current = match tokio::fs::read_to_string(&file).await {
        Ok(text) => text.trim().parse::<u64>().unwrap_or(1),
        Err(_) => 1,
    };

    let next = current.checked_add(1).expect("CRL number overflow");
    tokio::fs::write(&file, next.to_string()).await.unwrap();

    current
}

fn crl_number_file(ca_path: &CaDataPath) -> String {
    let mut path = ca_path.as_str().to_string();
    if !path.ends_with('/') {
        path.push('/');
    }
    path.push_str("crl_number");
    path
}
