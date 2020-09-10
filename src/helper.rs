pub fn check_project_integrity() {
    use std::fs;
    fs::read_dir("./.ast_config")
        .expect("Current directory is not a aws-serverless-toolkit project");
}
pub fn fetch_or_create_ast_config() -> Result<String, std::io::Error> {
    use std::fs;
    fs::read_to_string("./ast.yaml")
}
