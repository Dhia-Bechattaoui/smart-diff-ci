use serde::Deserialize;
use std::fs;
use regex::Regex;

#[derive(Deserialize, Default, Debug)]
pub struct ConfigData {
    pub test_patterns: Option<Vec<String>>,
}

pub struct Config {
    pub compiled_patterns: Vec<Regex>,
}

impl Config {
    pub fn load() -> Self {
        let mut raw_config = ConfigData::default();

        if let Ok(content) = fs::read_to_string("smart-diff.yaml") {
            if let Ok(parsed) = serde_yaml::from_str::<ConfigData>(&content) {
                raw_config = parsed;
            } else {
                println!("⚠️ Warning: Found smart-diff.yaml but failed to parse it. Falling back to defaults.");
            }
        }

        let mut compiled_patterns = Vec::new();
        
        if let Some(patterns) = raw_config.test_patterns {
            for pat in patterns {
                if let Ok(regex) = Regex::new(&pat) {
                    compiled_patterns.push(regex);
                } else {
                    println!("⚠️ Warning: Failed to compile regex '{}' from smart-diff.yaml", pat);
                }
            }
        }

        // If no custom patterns were found or valid, fallback to defaults
        if compiled_patterns.is_empty() {
            let defaults = vec![
                r".*_test\.dart$",
                r".*\.test\.ts$",
                r".*\.test\.js$",
                r".*\.spec\.ts$",
                r".*\.spec\.js$",
                r"^test_.*\.py$",
                r".*_test\.py$",
            ];
            for pat in defaults {
                compiled_patterns.push(Regex::new(pat).unwrap());
            }
        }

        Config { compiled_patterns }
    }
}
