use crate::{Error, Result};
use std::{env, str::FromStr, sync::OnceLock};

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| match Config::load_from_env() {
		Ok(config) => config,
		Err(ex) => panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"),
	})
}

#[allow(non_snake_case)]
pub struct Config {
	// -- Crypt
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,

	// -- Db
	pub DB_URL: String,

	// -- Web
	pub WEB_FOLDER: String,
}

impl Config {
	fn load_from_env() -> Result<Self> {
		Ok(Self {
			PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,
			TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
			TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
			DB_URL: get_env("SERVICE_DB_URL")?,
		})
	}
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
	base64_url::decode(&get_env(name)?)
		.map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
	let val = get_env(name)?;
	val.parse().map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env(name: &'static str) -> Result<String> {
	match env::var(name) {
		Ok(e) => Ok(e),
		Err(_) => Err(Error::ConfigMissingEnv(name)),
	}
}
