use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct ProjectConfig {
    pub project_info: HashMap<String, String>,
    pub dependencies: HashMap<String, String>,
    pub build_settings: HashMap<String, String>,
    pub runtime_settings: HashMap<String, String>,
    pub custom_settings: HashMap<String, String>,
}

pub fn parse_zom_file(file: File) -> Result<ProjectConfig, std::io::Error> {
    let reader = BufReader::new(file);

    let mut config = ProjectConfig {
        project_info: HashMap::new(),
        dependencies: HashMap::new(),
        build_settings: HashMap::new(),
        runtime_settings: HashMap::new(),
        custom_settings: HashMap::new(),
    };

    let mut current_section = String::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.starts_with("##") {
            current_section = trimmed[2..].trim().to_string();
        } else if trimmed.starts_with("-") {
            let parts: Vec<&str> = trimmed[1..].splitn(2, ':').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let key = parts[0].to_string();
                let value = parts[1].to_string();
                match current_section.as_str() {
                    "Project Info" => {
                        config.project_info.insert(key, value);
                    }
                    "Dependencies" => {
                        config.dependencies.insert(key, value);
                    }
                    "Build Settings" => {
                        config.build_settings.insert(key, value);
                    }
                    "Runtime Settings" => {
                        config.runtime_settings.insert(key, value);
                    }
                    "Custom Settings" => {
                        config.custom_settings.insert(key, value);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(config)
}
