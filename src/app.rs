use log::LevelFilter;
use once_cell::sync::OnceCell;

use crate::config::Config;

static VERBOSITY: OnceCell<LevelFilter> = OnceCell::new();
static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn set_global_verbosity(verbosity: LevelFilter) {
    VERBOSITY.set(verbosity).expect("could not set verbosity")
}

pub fn set_global_config(config: Config) {
    CONFIG.set(config).expect("could not set config")
}
