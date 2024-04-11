use clap::{Arg, Command};
use glob::glob;
use serde::Deserialize;
use serde_yaml;
use std::io::{self, ErrorKind};
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize)]
struct Config {
    files: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Project Summarizer")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Summarizes project files specified in a config file")
        .arg(
            Arg::new("base_path")
                .short('b')
                .long("base")
                .value_name("BASE_PATH")
                .help("Sets the base path for the project"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG_FILE")
                .help("Sets the config file"),
        )
        .get_matches();

    let binding = String::from(".");
    let base_path = matches.get_one::<String>("base_path").unwrap_or(&binding);
    let config_file = matches
        .get_one::<String>("config")
        .expect("A config file is required");

    let config_path = PathBuf::from(base_path).join(config_file);
    let config_contents = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_contents)?;

    let mut paths_summary = String::from("## Key Files Paths\n");
    let mut contents_summary = String::from("\n## Files Contents\n");

    for pattern in config.files {
        let full_pattern = PathBuf::from(base_path).join(pattern);
        match glob(&full_pattern.to_string_lossy()) {
            Ok(paths) => {
                for entry in paths {
                    if let Ok(path) = entry {
                        if path.is_file() {
                            paths_summary.push_str(&format!("- {}\n", path.display()));
                            let content = fs::read_to_string(&path)?;
                            contents_summary.push_str(&format!(
                                "\n### File: {}\n```\n{}\n```\n",
                                path.display(),
                                content
                            ));
                        }
                    }
                }
            }
            Err(e) => {
                return Err(
                    io::Error::new(ErrorKind::Other, format!("Glob pattern error: {}", e)).into(),
                )
            }
        }
    }

    println!("{}\n{}", paths_summary, contents_summary);

    Ok(())
}
