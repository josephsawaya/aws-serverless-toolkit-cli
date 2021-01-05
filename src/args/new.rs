pub fn new() {
    use colored::*;
    use std::process::Command;
    let name = dialoguer::Input::<String>::new()
        .with_prompt("Your Project Name")
        .interact()
        .expect("Expected an argument");
    let mut path_name = String::from("./");
    path_name.push_str(&name);
    const LINK: &str = "https://github.com/josephsawaya/aws-serverless-toolkit-template.git";
    if cfg!(windows) {
        let mut cmd = format!(
            "git clone {} {} & RMDIR .\\{}\\git /S",
            &LINK, &path_name, &path_name
        );
        let mut errmessage = format!(
            "{} {}",
            "error:".bright_red().bold(),
            "failed to clone or remove git from created folderr"
        );
        let mut child = Command::new("cmd")
            .args(&["/C", &cmd])
            .spawn()
            .expect(&errmessage[..]);
        child.wait().expect(&errmessage[..]);
        std::env::set_current_dir(&path_name).expect("failed to execute process");
        cmd = format!("git init & mkdir .ast_config");
        errmessage = format!(
            "{} {}",
            "error:".bright_red().bold(),
            "failed to init git repo or create ast_config folder"
        );
        child = Command::new("cmd")
            .args(&["/C", &cmd])
            .spawn()
            .expect(&errmessage[..]);
        child.wait().expect(&errmessage[..]);
    } else {
        let mut cmd = format!(
            "git clone {} {} ; rm -rf ./{}/.git",
            &LINK, &path_name, &path_name
        );
        let mut errmessage = format!(
            "{} {}",
            "error:".bright_red().bold(),
            "failed to clone or remove git from created folderr"
        );
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .expect(&errmessage[..]);
        child.wait().expect(&errmessage[..]);
        std::env::set_current_dir(&path_name).expect("failed to execute process");
        cmd = format!("git init ; mkdir .ast_config");
        errmessage = format!(
            "{} {}",
            "error:".bright_red().bold(),
            "failed to init git repo or create ast_config folder"
        );
        child = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .expect(&errmessage[..]);
        child.wait().expect(&errmessage[..]);
    };
}
