use crate::datatypes::DataType;
use std::{
    collections::HashMap,
    io::Error,
    };
pub struct Metdata {
    pub key: DataType,
    pub value: DataType,
}

pub struct Storage{
    vault: HashMap<[u8; 512], [u8; 512]>,
}

impl Storage {
    pub fn new() ->  Self{
        let mut vault: HashMap<[u8; 512], [u8; 512]> = HashMap::new();
        Storage {
            vault
        }
    }

    pub fn set(&mut self, key: [u8; 512], value: [u8; 512]) -> Result<(), Error> {
        self.vault.insert(key, value);
        Ok(())
    }
}