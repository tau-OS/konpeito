use std::path::PathBuf;

use konpeito::KeyStore;


// Loads the key from a SSH keyfile

fn main() {
    let keyfile = "examples/id_rsa";
    // Load the database with the example keyfile
    let ks = KeyStore::load_with_passwdfile("/tmp/konpeito-db-ssh", PathBuf::from(keyfile));
    ks.set("foo", b"bar".to_vec());
    assert_eq!(ks.get("foo"), Some(b"bar".to_vec()));
}