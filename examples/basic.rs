use konpeito::KeyStore;


fn main() {
    // Simply load the keystore with a passphrase
    let key = "very secret key";
    let ks = KeyStore::new(key, "/tmp/konpeito-db");

    ks.set("foo", b"bar".to_vec());

    assert_eq!(ks.get("foo"), Some(b"bar".to_vec()));
}