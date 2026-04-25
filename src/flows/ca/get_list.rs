use crate::app::AppContext;

pub async fn get_list(app: &AppContext) -> Vec<String> {
    crate::storage::ca::list_cas(&app.settings.get_config_path()).await
}
