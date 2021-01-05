pub mod args;
pub mod helper;
pub mod types;

fn main() {
    use colored::*;
    use std::env;
    let mut pattern = env::args();
    let command = pattern.nth(1);
    match command.as_deref() {
        Some("new") => args::new::new(),
        Some("add") => args::add::add(pattern.nth(0)),
        Some("clear") => args::clear::clear(),
        Some(_) => print!(
            "{} {}",
            "error:".bright_red().bold(),
            "Invalid Argument, run ast-cli with no arguments to see possible commands\n"
        ),
        None => print!(
            "ast-cli
A CLI for projects using aws-serverless-toolkit
    
USAGE:
    ast-cli [COMMAND]
        
    Commands are:
    new         Compile the current package
    add         Analyze the current package and report errors, but don't build object files
    clear       Remove the target directory\n"
        ),
    };
}
