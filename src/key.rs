use std::{path::PathBuf, collections::HashMap};

use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCrypt256};


pub(crate) struct KeyStore {
    crypt: MagicCrypt256,
    database: sled::Db,
}

impl KeyStore {
    pub fn new(key: &str, path: &str) -> Self {


        let path_string = shellexpand::tilde(path).to_string();
        let path = PathBuf::from(path_string);

        // make dir if it doesn't exist
        if !path.exists() {
            std::fs::create_dir_all(&path).unwrap();
        }
        KeyStore {
            crypt: new_magic_crypt!(key, 256),
            database: sled::open(path).expect("failed to open db"),
        }
    }
    pub fn load_with_passwdfile(database: &str, passwdfile: PathBuf) -> Self {
        // check if passwdfile exists
        if !passwdfile.exists() {
            panic!("key file `{}` does not exist", passwdfile.display());
        }

        let passwd = std::fs::read_to_string(passwdfile).expect("failed to read key file");
        Self::new(&passwd, database)
        //todo!()
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let key_bytes = self.crypt.encrypt_bytes_to_bytes(key.as_bytes());
        //println!("{:?}", key_bytes);
        let value = self.database.get(key_bytes).expect("failed to get value").expect("value not found");
        let b = self.crypt.decrypt_bytes_to_bytes(&value).ok().map_or_else(|| {
            println!("failed to decrypt value");
            None
        }, |v| Some(v));
        b
    }

    pub fn set(&self, key: &str, value: Vec<u8>) {
        let key_bytes = self.crypt.encrypt_str_to_bytes(key);
        let value_bytes = self.crypt.encrypt_bytes_to_bytes(&value);
        self.database.insert(key_bytes, value_bytes).expect("failed to set value");
    }

    pub fn list(&self) -> HashMap<String, Vec<u8>> {
        let kvs = self.database.iter();

        let mut map = HashMap::new();

        for kv in kvs {
            let key = &kv.as_ref().unwrap().0.to_vec();
            let key = self.crypt.decrypt_bytes_to_bytes(&key).unwrap();
            let key = key.clone().to_owned();
            let key = std::string::String::from_utf8_lossy(key.to_owned().as_slice()).to_string();
            let value = &kv.unwrap().1.to_vec();
            let value = self.crypt.decrypt_bytes_to_bytes(&value).unwrap();
            //let value = std::string::String::from_utf8(value).ok();
            //println!("{:?} = {:?}", key, value);
            map.insert(key, value);
        }
        map
    }

    pub fn delete(&self, key: &str) {
        let key_bytes = self.crypt.encrypt_str_to_bytes(key);
        self.database.remove(key_bytes).expect("failed to delete key");
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let a = KeyStore::load_with_passwdfile("konpeito-db", PathBuf::from("/home/cappy/.ssh/id_rsa"));
        a.set("c", "the mitochondria is the powerhouse of the cell".as_bytes().to_vec());
        let l = a.list();
        println!("{:?}", l);
        //a.delete("c");
        //let l = a.list();
        //println!("{:?}", l);
        let a = a.get("c");
        println!("{:?}", a);
    
    }
}