use std::sync::Arc;

use crate::app::AppContext;

pub async fn sync_vars_on_start(app: &Arc<AppContext>) {
    let temp_dir = app.settings.get_temp_dir();

    let temp_var_file = temp_dir.to_temp_vars_file();

    let vars_content = tokio::fs::read_to_string(temp_var_file.as_str()).await;

    if vars_content.is_err() {
        println!("Looks like no CA was generated yet. Skipping sync vars on start.");
    }

    let vars_content = vars_content.unwrap();

    let vars_path = app.get_vars_path();

    tokio::fs::write(vars_path, vars_content.as_bytes())
        .await
        .unwrap();
}
