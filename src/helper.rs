use colored::*;
use yaml_rust::Yaml;

pub fn check_project_integrity() {
    use std::fs;
    match fs::read_dir("./.ast_config") {
        Ok(_) => (),
        Err(_) => {
            println!(
                "{} {}",
                "error:".bright_red().bold(),
                "current directory is not an ast project"
            );
            std::process::exit(0)
        }
    }
}
pub fn fetch_ast_config() -> Result<String, std::io::Error> {
    use std::fs;
    fs::read_to_string("./ast.yaml")
}

pub fn fetch_names_from_config(config: &Yaml, element: String) -> std::vec::Vec<String> {
    let config = config.as_hash().unwrap();
    let mut res: std::vec::Vec<String> = std::vec::Vec::new();
    for (_, val) in config {
        let val = val.as_vec();
        match val {
            Some(x) => {
                for elem in x {
                    let b = elem.as_hash().unwrap();
                    let key = &Yaml::String(element.clone());
                    if b.contains_key(key) {
                        let name_key =
                            &Yaml::String(format!("{}Name", element.to_ascii_lowercase()));
                        let potential_name = b[key].as_hash().unwrap()[name_key].as_str();
                        match potential_name {
                            Some(x) => res.push(String::from(x)),
                            None => (),
                        }
                    }
                }
            }
            None => (),
        }
    }
    res
}
