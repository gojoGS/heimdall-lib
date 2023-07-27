use std::fs;

use log::info;

pub enum DataSource {
    Debug,
    String(String),
    File(String),
}

impl DataSource {
    pub fn get_contents_string(&self) -> String {
        return match self {
            Self::Debug => DEFAULT_LOG_STRING.to_string(),
            Self::File(path) => Self::get_contents_file(path),
            Self::String(content) => content.to_owned(),
        };
    }

    fn get_contents_file<S>(path: S) -> String
    where
        S: Into<String>,
    {
        let path_str = path.into();
        info!("Reading from: {}", path_str);
        fs::read_to_string(path_str).unwrap()
    }
}

const DEFAULT_LOG_STRING: &str = r#"*************
* JGfFinder *
*************
[INFO] Application started
[DEBUG] Initializing db
[INFO] Something important
[WARNING] JAR not found
[WARNING] JAR not found
[WARNING] JAR not found
[WARNING] JAR not found
[DEBUG] Try default JAR
[ERROR] Default JAR has a relship already
[INFO] Application started
[DEBUG] Initializing db
[INFO] Something important
[WARNING] JAR not found
[WARNING] JAR not found
[WARNING] JAR not found
[WARNING] JAR not found
[DEBUG] Try default JAR
[ERROR] Default JAR has a relship already
[INFO] Application started
[DEBUG] Initializing db
[INFO] Something important
[WARNING] JAR not found
[WARNING] JAR not found
[WARNING] JAR not found
[WARNING] JAR not found
[DEBUG] Try default JAR
[ERROR] Default JAR has a relship already
[INFO] Application started
[DEBUG] Initializing db
[INFO] Something important
[WARNING] JAR not found
[WARNING] JAR not found
[WARNING] JAR not found
[WARNING] JAR not found
[DEBUG] Try default JAR
[ERROR] Default JAR has a relship already"#;
