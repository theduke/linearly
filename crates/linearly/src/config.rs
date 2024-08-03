use std::path::PathBuf;

use anyhow::Context;

pub const LINEAR_API_KEY_SETTINGS_URL: &str = "https://linear.app/wasmer/settings/api";

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct UserConfig {
    pub token: Option<String>,
}

impl UserConfig {
    const FILENAME: &'static str = "config.toml";

    fn path() -> PathBuf {
        #[allow(deprecated)]
        std::env::home_dir()
            .expect("could not determine user home directory")
            .join(".config")
            .join("linearly")
            .join(Self::FILENAME)
    }

    pub fn load() -> Result<Option<Self>, anyhow::Error> {
        let path = Self::path();
        let content = match std::fs::read_to_string(&path) {
            Ok(v) => v,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Ok(None);
            }
            Err(err) => {
                anyhow::bail!(
                    "could not load config file at '{}': {}",
                    path.display(),
                    err
                );
            }
        };

        let config = toml::from_str(&content)
            .with_context(|| format!("Could not load config file at '{}'", path.display()))?;

        Ok(Some(config))
    }

    pub fn persist(data: &Self) -> Result<PathBuf, anyhow::Error> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Could not create directory: '{}'", parent.display()))?;
        }

        let data = toml::to_string_pretty(&data)?;
        std::fs::write(&path, data)
            .with_context(|| format!("could not write config file at '{}'", path.display()))?;

        Ok(path)
    }

    pub fn modify(modifier: impl FnOnce(&mut Self)) -> Result<(Self, PathBuf), anyhow::Error> {
        let mut config = Self::load()?.unwrap_or_default();
        modifier(&mut config);
        let path = Self::persist(&config)?;
        Ok((config, path))
    }

    pub fn update_token(token: String) -> Result<(Self, PathBuf), anyhow::Error> {
        Self::modify(move |config| config.token = Some(token))
    }
}
