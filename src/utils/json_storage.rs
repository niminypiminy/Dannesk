use dirs::config_dir;
use std::fs::{self, create_dir_all};
use std::path::PathBuf;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use std::io;

pub fn get_config_path(filename: &str) -> io::Result<PathBuf> {
    let path = config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not determine config directory"))?
        .join("Dannesk")
        .join(filename);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    Ok(path)
}

pub fn write_json<T: Serialize>(filename: &str, data: &T) -> io::Result<()> {
    let path = get_config_path(filename)?;
    let json = serde_json::to_string(data)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn read_json<T: DeserializeOwned>(filename: &str) -> io::Result<T> {
    let path = get_config_path(filename)?;
    let content = fs::read_to_string(path)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn update_json<T: Serialize + DeserializeOwned + std::default::Default>(
    filename: &str,
    update_fn: impl FnOnce(&mut T),
) -> io::Result<()> {
    let mut data = read_json(filename).or_else(|_| Ok::<T, std::io::Error>(T::default()))?;
    update_fn(&mut data);
    write_json(filename, &data)?;
    Ok(())
}

pub fn remove_json(filename: &str) -> io::Result<()> {
    let path = get_config_path(filename)?;
    fs::remove_file(path)?;
    Ok(())
}