use keyring::Result;
use crate::key_manager::KeyManager;
use serde::{Serialize, Deserialize};
use std::io::{self, Write};

pub struct StructKeyManager<T> {
    key_manager: KeyManager,
    _marker: std::marker::PhantomData<T>,
}

impl<T> StructKeyManager<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Default,
{
    /// Creates a new instance of StructKeyManager with the given system and key name.
    pub fn new(system_name: &str, key_name: &str) -> Self {
        StructKeyManager {
            key_manager: KeyManager::new(system_name, key_name),
            _marker: std::marker::PhantomData,
        }
    }

    /// Reads the value of a key from the keyring and deserializes it into a struct.
    pub fn read_key(&mut self) -> Result<T> {
        let json_value = self.key_manager.read_key()?;
        let struct_value: T = serde_json::from_str(&json_value).map_err(|e| keyring::Error::PlatformFailure(Box::new(e)))?;
        Ok(struct_value)
    }

    /// Reads the value of the key, and if it does not exist, prompts the user and saves the new key value in the keyring.
    pub fn read_or_request_key(&mut self) -> Result<T> {
        match self.read_key() {
            Ok(value) => Ok(value),
            Err(_) => {
                println!("The key was not found.");
                self.request_key()
            }
        }
    }

    /// Prompts the user to input each field of the struct and saves the serialized struct as the key value in the keyring.
    pub fn request_key(&mut self) -> Result<T> {
        // Converte a struct padrão para um objeto JSON
        let mut struct_map = serde_json::to_value(T::default())
            .map_err(|e| keyring::Error::PlatformFailure(Box::new(e)))?;

        // Atualiza cada campo do JSON com o valor do usuário
        if let serde_json::Value::Object(ref mut fields) = struct_map {
            for (field_name, field_value) in fields.iter_mut() {
                println!("Please enter the value for field '{}':", field_name);
                let mut input = String::new();
                io::stdout().flush().map_err(|e| keyring::Error::PlatformFailure(Box::new(e)))?;
                io::stdin().read_line(&mut input).map_err(|e| keyring::Error::PlatformFailure(Box::new(e)))?;
                let input = input.trim().to_string();

                // Tenta determinar o tipo do campo e realizar a conversão apropriada
                let new_value = if field_value.is_number() {
                    match input.parse::<i64>() {
                        Ok(num) => serde_json::Value::Number(num.into()),
                        Err(_) => {
                            eprintln!("Invalid input for field '{}'. Expected a number.", field_name);
                            continue; // Pede o valor novamente
                        }
                    }
                } else if field_value.is_boolean() {
                    match input.to_lowercase().as_str() {
                        "true" => serde_json::Value::Bool(true),
                        "false" => serde_json::Value::Bool(false),
                        _ => {
                            eprintln!("Invalid input for field '{}'. Expected true or false.", field_name);
                            continue; // Pede o valor novamente
                        }
                    }
                } else {
                    serde_json::Value::String(input)
                };

                // Atualiza o campo no mapa JSON
                *field_value = new_value;
            }
        }

        // Converte o objeto JSON para a struct T
        let struct_value: T = serde_json::from_value(struct_map)
            .map_err(|e| keyring::Error::PlatformFailure(Box::new(e)))?;

        // Armazena a struct no keyring
        self.store_key(&struct_value)?;
        Ok(struct_value)
    }



    /// Serializes the struct and stores it as the key value in the keyring.
    pub fn store_key(&mut self, value: &T) -> Result<()> {
        let json_value = serde_json::to_string(value).map_err(|e| keyring::Error::PlatformFailure(Box::new(e)))?;
        self.key_manager.store_key(&json_value)
    }

    /// Deletes the key value from the keyring.
    pub fn delete_key(&mut self) -> Result<()> {
        self.key_manager.delete_key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_struct_key_manager_new() {
        let manager: StructKeyManager<TestStruct> = StructKeyManager::new("key_manager_service", "test_struct_key");
        assert_eq!(manager.key_manager.key_name, "test_struct_key");
    }

    #[test]
    fn test_store_and_read_struct_key() {
        let mut manager: StructKeyManager<TestStruct> = StructKeyManager::new("key_manager_service", "test_struct_key");
        let test_value = TestStruct {
            field1: "value1".to_string(),
            field2: 42,
        };
        manager.store_key(&test_value).unwrap();
        let read_value = manager.read_key().unwrap();
        assert_eq!(read_value, test_value);
    }

    #[test]
    fn test_read_or_request_struct_key() {
        let mut manager: StructKeyManager<TestStruct> = StructKeyManager::new("key_manager_service", "test_struct_key");
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
    fn test_delete_struct_key() {
        let mut manager: StructKeyManager<TestStruct> = StructKeyManager::new("key_manager_service", "test_struct_key");
        let test_value = TestStruct {
            field1: "value1".to_string(),
            field2: 42,
        };
        manager.store_key(&test_value).unwrap();
        manager.delete_key().unwrap();
        let result = manager.read_key();
        assert!(result.is_err());
    }
}
