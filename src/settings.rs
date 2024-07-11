use std::env;

/// Get variable from environtment variable
///
/// # Examples
///
/// ```rust
/// use axum_askama::settings::get_setting;
/// use std::env;
///
/// let key = "TEST_ENVIRONTMENT_VARIABLE";
/// let val = "yes";
/// env::set_var(key, val);
/// let result = get_setting(key.to_string(), false);
/// assert_eq!(result, Some(val.to_string()));
/// ```
pub fn get_setting(key: String, required: bool) -> Option<String> {
    let var = env::var(key.clone());
    match var {
        Ok(data) => Some(data),
        Err(_) => {
            if required {
                panic!("env variable {} must be defined!", key.clone());
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_settings_found() {
        let key = "TEST_ENVIRONTMENT_VARIABLE";
        let val = "yes";
        env::set_var(key, val);
        let result = get_setting(key.to_string(), false);
        assert_eq!(result, Some(val.to_string()));
    }

    #[test]
    fn test_get_settings_not_found() {
        let result = get_setting("jcaocaco".to_string(), false);
        assert_eq!(result, None);
    }

    #[test]
    #[should_panic]
    fn test_get_settings_required() {
        get_setting("jcaocaco".to_string(), true);
    }
}
