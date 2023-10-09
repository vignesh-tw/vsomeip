use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
const DEFAULT_CONFIG_PATH: &str =  ".mig.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub authorization: String,
    pub project: String,
    pub slug: String
}

#[allow(dead_code)]
pub struct Manager{
    config_path: String
}

impl Manager {
    #[allow(dead_code)]
    pub fn new(custom_path: Option<&str>) -> Manager {
        match custom_path {
            Some(path) => {
                return Manager {
                    config_path: path.to_string()
                }
            },
            None => {
                return Manager { 
                    config_path: DEFAULT_CONFIG_PATH.to_string(),
                }
            }
        } 
    }

    #[allow(dead_code)]
    pub fn config_exist(&self) -> bool {
        return Path::new(&self.config_path).exists();
    }

    #[allow(dead_code)]
    pub fn read_config(&self) -> Config {
        let data = fs::read_to_string(&self.config_path).expect("failed to open config file");
        return serde_json::from_str(&data).unwrap();
    }

    #[allow(dead_code)]
    pub fn write_config(&self, authorization: String, project: String, slug: String) {
        let config = Config{authorization,project,slug};
        let content = serde_json::to_string(&config).unwrap();
        let mut f = File::create(&self.config_path).expect("Unable to create file");
        f.write_all(content.as_bytes()).expect("Unable to write data");
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use super::*;

    #[test]
    fn manager_has_default_config_path_by_default() {
        let manager = Manager::new(None);

        assert_eq!(manager.config_path, DEFAULT_CONFIG_PATH);
    }

    #[test]
    fn manager_config_path_can_be_set_when_initialised() {
        let custom_path = "a/path.json";
        let manager = Manager::new(Some(custom_path));

        assert_eq!(manager.config_path,custom_path);
    }

    #[test]
    fn config_exist_returns_false_if_config_file_does_not_exist() {
        let manager = Manager::new(None);
        let config_exist = manager.config_exist();

        assert!(!config_exist);
    }

    #[test]
    fn config_exist_returns_true_if_config_file_exists() {
        let custom_path = "test_config.json";
        let manager = Manager::new(Some(custom_path));
        File::create(&manager.config_path).unwrap();

        let config_exist = manager.config_exist();

        assert!(config_exist);

        fs::remove_file(custom_path).unwrap();
    }
   
    #[test]
    fn write_config_creates_config_file() {
        let custom_path = "test_config_2.json";
        let manager = Manager::new(Some(custom_path));
        let authorization = String::from("auth");
        let project = String::from("project");
        let slug = String::from("slug");
 
        manager.write_config(authorization, project, slug);

        let file_content = fs::read_to_string(custom_path).unwrap();

        let expected_content = "{\"authorization\":\"auth\",\"project\":\"project\",\"slug\":\"slug\"}";

        assert!(Path::new(custom_path).exists());
        assert_eq!(expected_content, file_content);

        fs::remove_file(custom_path).unwrap();
    }

    #[test]
    fn read_config_file_returns_config() {
        let custom_path = "test_config_3.json";
        let mut f = File::create(&custom_path).expect("Unable to create file");
        let auth = "entry";
        let project = "path";
        let slug = "dir";
        let content = format!("{{\"authorization\":\"{}\",\"project\":\"{}\",\"slug\":\"{}\"}}",auth,project,slug);
        f.write_all(content.as_bytes()).expect("Unable to write data");

        let manager = Manager::new(Some(custom_path));

        let config = manager.read_config();

        assert_eq!(auth,config.authorization);
        assert_eq!(project,config.project);
        assert_eq!(slug,config.slug);

        fs::remove_file(custom_path).unwrap();
    } 
}