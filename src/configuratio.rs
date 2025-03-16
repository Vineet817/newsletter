
use config;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}
impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
pub fn get_config() -> Result<Settings , config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()

}