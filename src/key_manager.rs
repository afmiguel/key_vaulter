use keyring::{Entry, Result};
use std::io::{self, Write};
#[allow(unused_imports)]
use std::env;

pub struct KeyManager {
    pub system_name: String,
    pub key_name: String,
    pub key_value: Option<String>,
}

impl KeyManager {
    /// Creates a new instance of KeyManager with the given key name.
    pub fn new(system_name: &str, key_name: &str) -> Self {
        KeyManager {
            system_name: system_name.to_string(),
            key_name: key_name.to_string(),
            key_value: None,
        }
    }

    /// Reads the value of a key from the keyring or environment variable (if feature `use_env_credentials` is enabled).
    ///
    /// Priority of key lookup:
    /// 1. **Environment Variable**: If the feature `use_env_credentials` is enabled, it will first try to read the key from the environment variables.
    /// 2. **Keyring**: If the key is not in the environment variables, it will then try to read it from the keyring.
    pub fn read_key(&mut self) -> Result<String> {
        // Se a feature `use_env_credentials` estiver habilitada, tente ler da variável de ambiente
        #[cfg(feature = "use_env_credentials")]
        {
            // println!("Feature `use_env_credentials` is enabled.");
            if let Ok(env_value) = env::var(&self.key_name) {
                self.key_value = Some(env_value.clone());
                return Ok(env_value);
            }
        }

        // Se não estiver na variável de ambiente, lê do keyring
        let entry = Entry::new(&self.system_name, &self.key_name)?;
        let password = entry.get_password()?;
        self.key_value = Some(password.clone());
        Ok(password)
    }

    /// Reads the value of the key, and if it does not exist, prompts the user and saves the new key value in the keyring.
    pub fn read_or_request_key(&mut self) -> Result<String> {
        match self.read_key() {
            Ok(value) => Ok(value),
            Err(_) => {
                println!("The key was not found.");
                self.request_key()
            }
        }
    }

    /// Prompts the user and saves the new key value in the keyring.
    pub fn request_key(&mut self) -> Result<String> {
        println!("Please enter the value for key {}:", self.key_name);
        let mut input = String::new();
        io::stdout().flush().map_err(|e| keyring::Error::PlatformFailure(Box::new(e)))?;
        io::stdin().read_line(&mut input).map_err(|e| keyring::Error::PlatformFailure(Box::new(e)))?;
        let input = input.trim().to_string();
        self.store_key(&input)?;
        Ok(input)
    }

    /// Stores the key value in the keyring.
    pub fn store_key(&mut self, value: &str) -> Result<()> {
        let entry = Entry::new(&self.system_name, &self.key_name)?;
        entry.set_password(value)?;
        self.key_value = Some(value.to_string());
        Ok(())
    }

    /// Deletes the key value from the keyring.
    pub fn delete_key(&mut self) -> Result<()> {
        let entry = Entry::new(&self.system_name, &self.key_name)?;
        entry.delete_credential()?;
        self.key_value = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_manager_new() {
        let test_key_name = "test_key1";
        let manager = KeyManager::new("key_manager_service", test_key_name);
        assert_eq!(manager.key_name, test_key_name);
        assert!(manager.key_value.is_none());
    }

    #[test]
    fn test_store_and_read_key() {
        let mut manager = KeyManager::new("key_manager_service", "test_key2");
        let test_value = "test_value";
        match manager.read_key() {
            Ok(_) => {
                manager.delete_key().unwrap();
            },
            Err(e) => {
                assert_eq!(format!("{:?}", e), "Error(PlatformFailure(Other(\"No such file or directory (os error 2)\")))");
            }
        }
        manager.store_key(test_value).unwrap();
        let read_value = manager.read_key().unwrap();
        assert_eq!(read_value, test_value);
    }

    #[test]
    fn test_read_or_request_key() {
        let mut manager = KeyManager::new("key_manager_service", "test_key3");
        match manager.read_key() {
            Ok(value) => {
                assert_eq!(manager.read_or_request_key().unwrap(), value);
            },
            Err(_) => {
                // Test input is not automated in this example.
                // To test this function, you would need to simulate stdin input.
            }
        }
    }

    #[test]
    fn test_delete_key() {
        let mut manager = KeyManager::new("key_manager_service", "test_key4");
        manager.store_key("test_value").unwrap();
        manager.delete_key().unwrap();
        let result = manager.read_key();
        assert!(result.is_err());
    }

    #[cfg(feature = "use_env_credentials")]
    #[test]
    fn test_read_key_from_env_variable() {
        env::set_var("TEST_KEY_ENV", "value_from_env");
        let mut manager = KeyManager::new("key_manager_service", "TEST_KEY_ENV");

        let read_value = manager.read_key().unwrap();
        assert_eq!(read_value, "value_from_env");

        env::remove_var("TEST_KEY_ENV");
    }

}
