use std::{
    env,
    path::{Path, PathBuf},
};

#[cfg(unix)]
const ENV_HOME: &str = "HOME";

#[cfg(windows)]
const ENV_HOME: &str = "USERPROFILE";

const CONFIG_FILE: &str = ".cmish-config.json";
const HISTORY_FILE: &str = ".cmish-history";

pub fn get_config_path() -> PathBuf {
    get_file_path(CONFIG_FILE)
}

pub fn get_history_path() -> PathBuf {
    get_file_path(HISTORY_FILE)
}

fn get_file_path(file: &str) -> PathBuf {
    Path::new(&env::var(ENV_HOME).unwrap()).join(file)
}
