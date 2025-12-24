use key_vaulter::struct_key_manager::StructKeyManager;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct MyStruct {
    #[serde(rename = "NOME USUÁRIO")]
    username: String,
    #[serde(rename = "IDADE")]
    age: u32,
}

fn main() {
    let mut manager: StructKeyManager<MyStruct> =
        StructKeyManager::new("my_system", "user_profile");

    match manager.read_key() {
        Ok(_) => {
            manager.delete_key().unwrap();
        }
        Err(_) => {}
    }

    match manager.read_or_request_key() {
        Ok(value) => {
            println!(
                "Successfully retrieved the struct from keyring: {:?}",
                value
            );
        }
        Err(err) => {
            eprintln!("Failed to read or request key from keyring: {:?}", err);
        }
    }

    manager.delete_key().unwrap();
}
