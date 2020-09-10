pub fn clear() {
    crate::helper::check_project_integrity();
    std::fs::write("./ast.yaml", "").expect("Unable to clear aws-serverless-toolkit config file");
}
