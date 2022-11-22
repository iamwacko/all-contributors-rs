use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn config() -> crate::Config {
    if PathBuf::from(".all-contributorsrc").exists() {
        let reader = BufReader::new(File::open(".all-contributorsrc").expect("Error while opening configuration file"));
        let config_data: crate::Config = serde_json::from_reader(reader).expect("Error while reading configuration file");
        return config_data
    } else {
        panic!("Run all-contributors-rs init before anything else");
    }
}
