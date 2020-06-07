use color_eyre::{Help, Report};
use config::{Config, ConfigError, Environment, File};
use eyre::WrapErr;
use serde::Deserialize;
use std::env;
use std::fs;
use std::net::IpAddr;
use std::sync::RwLock;
use tracing::instrument;

static CONFIG_FILE_DEFAULTS: &str = "config/defaults.hjson";
static CONFIG_FILE: &str = "config/config.hjson";

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub setup: Option<Setup>,
    pub database: Database,
    pub hostname: String,
    pub bind: IpAddr,
    pub port: u16,
    pub jwt_secret: String,
    pub front_end_dir: String,
    pub rate_limit: RateLimitConfig,
    pub email: Option<EmailConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Setup {
    pub admin_username: String,
    pub admin_password: String,
    pub admin_email: Option<String>,
    pub site_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub message: i32,
    pub message_per_second: i32,
    pub post: i32,
    pub post_per_second: i32,
    pub register: i32,
    pub register_per_second: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_login: Option<String>,
    pub smtp_password: Option<String>,
    pub smtp_from_address: String,
    pub use_tls: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub database: String,
    pub pool_size: u32,
}

lazy_static! {
    static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings::init().wrap_err("Unable to read config")
    .suggestion("try using a file that exists next time")
    .unwrap());
}

impl Settings {
    /// Reads config from the files and environment.
    /// First, defaults are loaded from CONFIG_FILE_DEFAULTS, then these values can be overwritten
    /// from CONFIG_FILE (optional). Finally, values from the environment (with prefix AGORA) are
    /// added to the config.
    #[instrument]
    fn init() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name(CONFIG_FILE_DEFAULTS))?;

        s.merge(File::with_name(CONFIG_FILE).required(false))?;

        // Add in settings from the environment (with a prefix of AGORA)
        // Eg.. `AGORA_DEBUG=1 ./target/app` would set the `debug` key
        // Note: we need to use double underscore here, because otherwise variables containing
        //       underscore cant be set from environmnet.
        // https://github.com/mehcode/config-rs/issues/73
        s.merge(Environment::with_prefix("AGORA").separator("__"))?;

        s.try_into()
    }

    /// Returns the config as a struct.
    #[instrument]
    pub fn get() -> Self {
        SETTINGS.read().unwrap().to_owned()
    }

    /// Returns the postgres connection url. If AGORA_DATABASE_URL is set, that is used,
    /// otherwise the connection url is generated from the config.
    pub fn get_database_url(&self) -> String {
        match env::var("AGORA_DATABASE_URL") {
            Ok(url) => url,
            Err(_) => format!(
                "postgres://{}:{}@{}:{}/{}",
                self.database.user,
                self.database.password,
                self.database.host,
                self.database.port,
                self.database.database
            ),
        }
    }

    pub fn api_endpoint(&self) -> String {
        format!("{}/api/v1", self.hostname)
    }

    #[instrument]
    pub fn read_config_file() -> Result<String, Report> {
        fs::read_to_string(CONFIG_FILE)
            .wrap_err("Unable to read config")
            .suggestion("make sure config/config.hjson exist")
    }

    #[instrument]
    pub fn save_config_file(data: &str) -> Result<String, Report> {
        fs::write(CONFIG_FILE, data)?;

        // Reload the new settings
        // From https://stackoverflow.com/questions/29654927/how-do-i-assign-a-string-to-a-mutable-static-variable/47181804#47181804
        let mut new_settings = SETTINGS.write().unwrap();
        *new_settings = Settings::init()
            .wrap_err("Unable to read config")
            .suggestion("try using a file that exists next time")
            .unwrap();

        Self::read_config_file()
    }
}
