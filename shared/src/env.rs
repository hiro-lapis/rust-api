use std::env;
use strum::EnumString;

#[derive(Default, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    #[default]
    Development,
    Production,
}

pub fn which() -> Environment {
    // set default_env based on debug_assertions attribute on build cmd
    #[cfg(debug_assertions)]
    let default_env = Environment::Development;
    #[cfg(not(debug_assertions))]
    let default_env = Environment::Production;

    // read ENV or debug_assertions
    match env::var("ENV") {
        Err(_) => default_env,
        Ok(v) => v.parse().unwrap_or(default_env),
    }
}
