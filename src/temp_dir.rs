pub struct TempDir {
    path: String,
}

impl TempDir {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }

    pub fn to_temp_vars_file(&self) -> String {
        format!("{}/vars", self.path)
    }

    pub fn into_string(self) -> String {
        self.path
    }
}
