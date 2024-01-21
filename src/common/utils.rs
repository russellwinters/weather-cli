use std::env;

pub fn load_env_var(str: &str) -> Result<String, ()> {
    let variable: Result<String, env::VarError> = env::var(str);

    match variable {
        Ok(data) => Ok(data),
        Err(e) => {
            eprintln!("Error getting API Key {}: {}", str, e);
            Err(())
        }
    }
}
