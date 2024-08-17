use crate::app::AppContext;

pub async fn get_list(app: &AppContext) -> Vec<String> {
    let ca_path = app.settings.get_config_path();
    println!("ca_path: '{}'", ca_path.as_str());

    let mut read_handle = tokio::fs::read_dir(ca_path.as_str()).await.unwrap();

    let mut result = Vec::new();

    while let Some(dir_entry) = read_handle.next_entry().await.unwrap() {
        match dir_entry.file_type().await.unwrap() {
            t if t.is_dir() => {
                let file_name = dir_entry.file_name().into_string().unwrap();
                result.push(file_name);
            }
            _ => {}
        }
    }

    result
}
