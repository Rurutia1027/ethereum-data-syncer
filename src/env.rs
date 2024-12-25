// env.rs is responsible for managing environment variables by loading them from the system environment and a local .env file.
// The `get_env_var` function first attempts to retrieve the requested key from the system environment variables.
// If the key is not found in the system environment, it will then attempt to load the value from the local .env file.
//
// This environment configuration is shared across different modules, providing thread-safe access to laoded
// variables such as API_KEY (e.g., MetaMask, Alchemy), DataSource URLs(e.g., RDBMS, Redis, GraphQL),
// and other configuration variables, including those required by docker-compose.yml.
//
// **Security Note**: API keys stored in this configuration might face security risks for purchased API_KEYs, including potential leakage.
// - For local development, it's acceptable to store keys in the .env file.
// - In CI/CD environments (e.g., GitHub Actions), it's recommended to use GitHub Secrets to prevent accidential exposure of sensitive data.

use core::panic;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref ENV_CONFIG: EnvConfig = get_env_config();
}

pub fn get_env_var(config_key: &str) -> Option<String> {
    // first search environment variables
    match env::var(config_key) {
        Ok(config_value) => Some(config_value),
        Err(e) => {
            // if not matched config_key in environment variables, continue with local .env
            dotenv().ok();
            env::var(config_key).ok()
        }
        Err(e) => panic!("Error retrieving environment variable: {}", e),
    }
}

pub fn get_env_bool(key: &str) -> Option<bool> {
    get_env_var(key).map(|var| match var.to_lowercase().as_str() {
        "true" => true,
        "false" => false,
        "t" => true,
        "f" => false,
        "1" => true,
        "0" => false,
        str => {
            panic!("invalid bool value {str} for {key}")
        }
    })
}

pub fn get_env_config() -> EnvConfig {
    EnvConfig {
        alchemy_url: get_env_var("ALCHEMY_URL").expect("ALCHEMY_URL is required!"),
        alchemy_api_key: get_env_var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY is required!"),
        alchemy_rpc_endpoint: get_env_var("ALCHEMY_RPC_ENDPOINT")
            .expect("ALCHEMY_RPC_ENDPOINT is required!"),

        metamask_url: get_env_var("METAMASK_URL").expect("METAMASK_URL is required!"),
        metamask_api_key: get_env_var("METAMASK_API_KEY").expect("METAMASK_API_KEY is required!"),
        metamast_rpc_endpoint: get_env_var("METAMASK_RPC_ENDPOINT")
            .expect("METAMASK_RPC_ENDPOINT is required!"),
    }
}

pub struct EnvConfig {
    pub alchemy_url: String,
    pub alchemy_rpc_endpoint: String,
    pub alchemy_api_key: String,

    pub metamask_url: String,
    pub metamast_rpc_endpoint: String,
    pub metamask_api_key: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_alchymy_url() {
        let config_key = "ALCHEMY_API_KEY";
        let ret = get_env_var(config_key).expect("ALCHEMY_API_KEY is required!");
        assert!(ret.len() > 0);

        let config_key = "ALCHEMY_URL";
        let ret = get_env_var(&config_key).expect("ALCHEMY_URL is required!");
        assert!(ret.len() > 0);

        let config_key = "ALCHEMY_RPC_ENDPOINT";
        let ret = get_env_var(&config_key).expect("ALCHEMY_RPC_ENDPOINT is required!");
        assert!(ret.len() > 0);
    }

    #[test]
    fn test_create_and_use_env_config() {
        let env_config = get_env_config();
        let ret = env_config.alchemy_url;
        assert!(ret.len() > 5);

        let ret = env_config.alchemy_api_key;
        assert!(ret.len() > 5);

        let ret = env_config.alchemy_rpc_endpoint;
        assert!(ret.len() > 5);
    }

    #[test]
    fn test_lazy_static_instance_of_env_config() {
        let ret = ENV_CONFIG.alchemy_url.as_str();
        assert!(ret.len() > 5);

        let ret = ENV_CONFIG.alchemy_api_key.as_str();
        assert!(ret.len() > 5);

        let ret = ENV_CONFIG.alchemy_rpc_endpoint.as_str();
        assert!(ret.len() > 5);
    }
}
