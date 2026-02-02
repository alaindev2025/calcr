use serde::Deserialize;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {}

#[allow(dead_code)]
impl Config {
    fn get_path() -> PathBuf {
        dirs::config_dir().unwrap().join("prueba/config.toml")
    }

    pub fn load() -> io::Result<Config> {
        Self::default()?;
        let s = fs::read_to_string(Self::get_path())?;
        let config: Config = toml::from_str(&s).unwrap();

        Ok(config)
    }

    fn default() -> io::Result<()> {
        let content = String::from("precise = 2");

        if !Self::get_path().exists()
            && let Some(parent) = Self::get_path().parent()
        {
            fs::create_dir_all(parent)?;
            let mut buf = fs::File::create(Self::get_path())?;
            buf.write_all(content.as_bytes())?;
        }

        Ok(())
    }
}
