use std::{path::PathBuf, collections::HashMap, io::BufReader};

use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCrypt256};

use osshkeys::{KeyPair};

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

        eprintln!("Loading encryption key");
        let crypt = new_magic_crypt!(key, 256);
        eprintln!("Loading database");
        let database = sled::open(path).expect("failed to open db");

        // println!("{}", key);
        KeyStore {
            crypt,
            database,
        }
    }
    pub fn load_with_passwdfile(database: &str, passwdfile: PathBuf) -> Self {
        // check if passwdfile exists
        if !passwdfile.exists() {
            panic!("key file `{}` does not exist", passwdfile.display());
        }

        let passwd = std::fs::read_to_string(passwdfile).expect("failed to read key file");
        eprintln!("Reading password file");

        //let keypair = KeyPair::from_keystr(&passwd, None).expect("failed to parse key file");
        //let pass = keypair.serialize_pem(None).expect("failed to serialize key file");

        let pass = {
            // check if we can parse thing as ssh key
            if let Ok(keyphrase) = KeyPair::from_keystr(&passwd, None) {
                keyphrase.serialize_pem(None).expect("failed to serialize key file")
            } else {
                // plain text password
                passwd
            }
        };

        //println!("{}", pass);
        Self::new(&pass, database)
        //todo!()
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let key_bytes = self.crypt.encrypt_bytes_to_bytes(key.as_bytes());
        //println!("{:?}", key_bytes);
        let value = self.database.get(key_bytes).expect("failed to get value").expect("value not found");
        let mut reader = BufReader::new(value.as_ref());

        self.crypt.decrypt_reader_to_bytes(&mut reader).ok().map_or_else(|| {
            println!("failed to decrypt value");
            None
        }, Some)
    }

    pub fn set(&self, key: &str, value: Vec<u8>) {
        let key_bytes = self.crypt.encrypt_str_to_bytes(key);
        let mut reader = BufReader::new(value.as_slice());
        let value_bytes = self.crypt.encrypt_reader_to_bytes(&mut reader).expect("failed to encrypt value");
        self.database.insert(key_bytes, value_bytes).expect("failed to set value");
    }

    pub fn list(&self) -> HashMap<String, Vec<u8>> {
        let kvs = self.database.iter();
        eprintln!("Loading keys...");

        let mut map = HashMap::new();

        for kv in kvs {
            let key = &kv.as_ref().unwrap().0.to_vec();
            let key = self.crypt.decrypt_bytes_to_bytes(&key).expect("Failed to decrypt key-value store");
            let key = key.clone().to_owned();
            let key = std::string::String::from_utf8_lossy(key.to_owned().as_slice()).to_string();
            let value = &kv.unwrap().1.to_vec();

            let mut reader = BufReader::new(value.as_slice());
            eprintln!("Decrypting {}", key);
            let value = self.crypt.decrypt_reader_to_bytes(&mut reader).unwrap();
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
        let string = "hello world";
        a.set("c", string.as_bytes().to_vec());
        //let l = a.list();
        //println!("{:?}", l);
        //a.delete("c");
        //let l = a.list();
        //println!("{:?}", l);
        let a = a.get("c");
        assert!(a.is_some());
        assert_eq!(a.unwrap(), string.as_bytes().to_vec());
    
    }
}