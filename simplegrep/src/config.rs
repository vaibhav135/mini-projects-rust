use super::custom_err::GrepError;

#[derive(Debug)]
pub struct GrepConfig {
    pub searchstring: String,
    pub filepath: String,
}

impl GrepConfig {
    pub fn build(args: Vec<String>) -> Result<GrepConfig, GrepError> {
        let searchstring = args
            .get(1)
            .ok_or(GrepError::MissingSearchString)?
            .to_string();

        let filepath = args.get(2).ok_or(GrepError::MissingFilePath)?.to_string();

        Ok(GrepConfig {
            searchstring,
            filepath,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_searchstring() {
        let test_args = vec![""].iter().map(|val| val.to_string()).collect();
        let config = GrepConfig::build(test_args);
        let someerr = config.unwrap_err();
        assert_eq!(someerr, GrepError::MissingSearchString);
    }

    #[test]
    fn invalid_filepath() {
        let test_args = vec!["", "you"].iter().map(|val| val.to_string()).collect();
        let config = GrepConfig::build(test_args);
        let someerr = config.unwrap_err();
        assert_eq!(someerr, GrepError::MissingFilePath);
    }

    #[test]
    fn is_valid_config() {
        let filepath = "simplegrep/assets/testfile.txt";
        let test_args = vec!["", "you", filepath]
            .iter()
            .map(|val| val.to_string())
            .collect();
        let config = GrepConfig::build(test_args).unwrap();
        assert_eq!(config.searchstring, "you");
        assert_eq!(config.filepath, filepath);
    }
}
