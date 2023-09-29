use std::fs;
use serde::Deserialize;
use std::path::Path;
use std::io::{Error,ErrorKind};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    authorization: String,
}

pub struct Session {
    config_path: String,
}

impl Session {
    pub fn from(config_path: &String) -> Session {
        return Session { config_path: config_path.clone() }
    }

    pub fn get_auth(&self) -> Result<String, Error> {
        let path = Path::new(&self.config_path);
        if !path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "failed to find config")); 
        };

        let data = fs::read_to_string(&self.config_path).expect("failed to open config file");
        let config: Config = match serde_json::from_str(&data) {
            Ok(c) => c,
            Err(_) => {
                let auth_err = format!("authorization not present in {}", &self.config_path);
                return Err(Error::new(ErrorKind::NotFound, auth_err))
            },
        };

        if config.authorization.is_empty() {
            let auth_err = format!("authorization not set in {}", &self.config_path);
            return Err(Error::new(ErrorKind::NotFound, auth_err)) 
        }

        Ok(config.authorization)
    }
}



#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    use super::*;

    #[test]
    fn get_auth_fails_if_config_does_not_exist() {
        let config_path = String::from("test_1_config.json");
        let session = Session::from(&config_path);
        let actual_err = session.get_auth().unwrap_err();

        assert_eq!(actual_err.kind(), ErrorKind::NotFound);
        assert_eq!(actual_err.into_inner().unwrap().to_string(), format!("failed to find config"))   
    }

    #[test]
    fn get_auth_fails_if_auth_is_not_found() {
        let config_path = String::from("test_2_config.json");
        let session = Session::from(&config_path);
        create_config(&config_path, None);

        let result = session.get_auth(); 
        delete_config(&config_path);
        let actual_err = result.unwrap_err();

        assert_eq!(actual_err.kind(), ErrorKind::NotFound);
        assert_eq!(actual_err.into_inner().unwrap().to_string(), format!("authorization not present in {}", &config_path));
    }

    #[test]
    fn get_auth_fails_if_auth_is_empty() {
        let config_path = String::from("test_3_config.json");
        let session = Session::from(&config_path);
        let auth = String::from("");
        create_config(&config_path, Some(auth));

        let result = session.get_auth(); 
        delete_config(&config_path);
        let actual_err = result.unwrap_err();

        assert_eq!(actual_err.kind(), ErrorKind::NotFound);
        assert_eq!(actual_err.into_inner().unwrap().to_string(), format!("authorization not set in {}", &config_path));
    }

    #[test]
    fn get_auth_returns_config_authorization() {
        let config_path = String::from("test_4_config.json");
        let session = Session::from(&config_path);
        let auth = String::from("authorized");
        create_config(&config_path, Some(auth.clone()));

        let result = session.get_auth(); 
        delete_config(&config_path);

        assert_eq!(auth, result.unwrap()); 
    }

    fn create_config(config_path: &String,authorization: Option<String>) {
        let mut file = File::create(config_path).unwrap();
        let content: String = match authorization {
            Some(auth) => format!("{{ \"authorization\": \"{}\" }}", auth),
            None => String::from("{}"),

        };
        _ = file.write_all(content.as_bytes());
    }

    fn delete_config(config_path: &String) {
        fs::remove_file(config_path).unwrap();
    }
}
