# Key Vaulter

**Secure Key Management in Rust**

---

## Overview

The **Key Vaulter** library is a Rust crate designed to facilitate the secure management of keys and serialized structs using system keyrings. It enables you to securely store, retrieve, and delete keys and structured data from the system's keyring with support for Windows, macOS, and Linux. Additionally, the crate offers support for reading keys from environment variables as a fallback.

---

## Features

- **Key Management**: Store, read, update, and delete keys in the system keyring.
- **Struct Serialization**: Store entire Rust structs as JSON in the keyring.
- **Environment Variable Support**: Optionally read keys from environment variables (requires `env_key` feature).
- **Cross-Platform**: Supports Windows, macOS, and Linux.

---

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
key_vaulter = "0.1.0"
```

To enable the **environment variable support**, add the following feature flag to your `Cargo.toml`:

```toml
[dependencies]
key_vaulter = { version = "0.1.0", features = ["env_key"] }
```

---

## Usage

### 1. **Basic Key Management**

```rust
use key_vaulter::key_manager::KeyManager;

fn main() {
    let mut manager = KeyManager::new("my_service", "my_key");

    // Store a key
    manager.store_key("my_secret_value").unwrap();

    // Read the key
    let value = manager.read_key().unwrap();
    println!("Retrieved key value: {}", value);

    // Delete the key
    manager.delete_key().unwrap();
}
```

---

### 2. **Struct Key Management**

The **StructKeyManager** allows you to manage entire Rust structs in the keyring. This is useful when you need to store and retrieve multiple pieces of information as a single entity.

```rust
use key_vaulter::struct_key_manager::StructKeyManager;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct MyStruct {
    username: String,
    age: u32,
}

fn main() {
    let mut manager: StructKeyManager<MyStruct> = StructKeyManager::new("my_system", "user_profile");

    // Store a struct
    let user_profile = MyStruct {
        username: "john_doe".to_string(),
        age: 30,
    };
    manager.store_key(&user_profile).unwrap();

    // Read the struct
    let value = manager.read_key().unwrap();
    println!("Retrieved struct: {:?}", value);

    // Delete the key
    manager.delete_key().unwrap();
}
```

---

### 3. **Reading from Environment Variables**

If the `env_key` feature is enabled, **Key Vaulter** will prioritize reading the key from the environment variable before attempting to read from the keyring.

```rust
use key_vaulter::key_manager::KeyManager;
use std::env;

fn main() {
    env::set_var("MY_ENV_KEY", "value_from_env");
    let mut manager = KeyManager::new("my_service", "MY_ENV_KEY");

    // If the environment variable exists, it will be used instead of the keyring.
    let value = manager.read_key().unwrap();
    println!("Retrieved key value: {}", value);

    env::remove_var("MY_ENV_KEY");
}
```

---

## API Reference

### **KeyManager**

#### **Methods**

- **new(system_name: &str, key_name: &str) -> KeyManager**
  - Creates a new instance of `KeyManager` for a specific system and key name.

- **read_key(&mut self) -> Result<String>**
  - Reads the value of the key from the keyring or environment variable.

- **read_or_request_key(&mut self) -> Result<String>**
  - Reads the key. If the key is not found, it prompts the user for input and stores it in the keyring.

- **store_key(&mut self, value: &str) -> Result<()>**
  - Stores a new key in the keyring.

- **delete_key(&mut self) -> Result<()>**
  - Deletes the key from the keyring.

---

### **StructKeyManager**

#### **Methods**

- **new(system_name: &str, key_name: &str) -> StructKeyManager<T>**
  - Creates a new instance of `StructKeyManager` for a specific system and key name.

- **read_key(&mut self) -> Result<T>**
  - Reads and deserializes the struct stored in the keyring.

- **read_or_request_key(&mut self) -> Result<T>**
  - Reads the key. If the key is not found, it prompts the user to input values for each struct field and stores it in the keyring.

- **store_key(&mut self, value: &T) -> Result<()>**
  - Serializes and stores a struct in the keyring.

- **delete_key(&mut self) -> Result<()>**
  - Deletes the key from the keyring.

---

## Testing

Run the tests using:

```bash
cargo test
```

The tests cover the following scenarios:
- Storing and reading a simple key
- Storing and reading a serialized struct
- Deleting keys from the keyring
- Reading from environment variables (when the `env_key` feature is enabled)

---

## Example Projects

Here are some example projects that utilize **Key Vaulter**:

1. **User Profile Storage**: Store and retrieve user profiles with names, emails, and preferences.
2. **API Key Management**: Securely store API keys for services like GitHub, AWS, or other API-driven tools.
3. **Environment Variable Override**: Use environment variables to override keyring values for flexibility in development and production environments.

---

## Contributing

1. Fork the project.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a Pull Request.

---

## License

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.

---

## Acknowledgments

- **Keyring Crate**: Uses the `keyring` crate to manage key storage.
- **Serde Crate**: For serialization and deserialization of structs.

---

Start using **Key Vaulter** to safely and securely manage your secrets and serialized data in Rust applications today!
