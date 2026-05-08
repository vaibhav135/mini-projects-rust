use std::{env, error::Error};

pub struct GrepConfig {
    pub searchstring: String,
    pub filepath: String,
}

impl GrepConfig {
    pub fn build() -> Result<GrepConfig, Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();
        let searchstring = args
            .get(1)
            .expect("searchstring cannot be empty")
            .to_string();

        let filepath = args.get(2).expect("filepath cannot be empty").to_string();

        println!("{searchstring}  and {filepath}");
        Ok(GrepConfig {
            searchstring,
            filepath,
        })
    }
}
