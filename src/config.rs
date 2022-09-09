use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, path::PathBuf};

use pg_branch::git;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub git_main_branch: String,
    pub template_db: String,
    pub current_db_branch: String,
    pub load_dot_env: bool,
    // branches as: {branch_name: {db_name, db_create_time}}
    pub branches: HashMap<String, HashMap<String, i64>>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::_path()?;
        let config: Config = confy::load_path(config_path)?;
        Ok(config)
    }
    pub fn save(&self) -> Result<()> {
        let config_path = Self::_path()?;
        confy::store_path(config_path, self)?;

        Ok(())
    }

    fn _path() -> Result<PathBuf> {
        let repo_root = git::repo_root_path(None)?;
        Ok(repo_root.join(".pg_branch/config.toml"))
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").into(),
            git_main_branch: "".into(),
            template_db: "".into(),
            current_db_branch: "".into(),
            branches: HashMap::new(),
            load_dot_env: false,
        }
    }
}
