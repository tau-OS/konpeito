use color_eyre::Result;
use base64::prelude::*;
pub fn get_key(name: &str, service: &str) -> Result<Vec<u8>> {
    let entry = keyring::Entry::new(service, name)?;
    let b = entry.get_password()?;
    // decode from base64
    BASE64_STANDARD.decode(b.as_bytes()).map_err(Into::into)
}

pub fn set_key(name: &str, value: &[u8], service: &str) -> Result<()> {
    let entry = keyring::Entry::new(service, name)?;
    entry.set_password(&BASE64_STANDARD.encode(value))?;
    Ok(())
}

pub fn del_key(name: &str, service: &str) -> Result<()> {
    keyring::Entry::new(service, name)?.delete_password()?;
    Ok(())
}

pub fn list_keys(service: &str) -> Result<Vec<String>> {
    let res = keyring_search::Search::new().expect("failed to create search")
        .by_service(service).expect("failed to set service");
    
    let mut r = res.iter().filter_map(|(_, val)| {
        val.get("username").map(|v| v.to_string())
    }).collect::<Vec<String>>();
    
    r.sort();
    
    Ok(r)
}