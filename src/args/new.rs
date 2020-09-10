pub fn new() {
    use std::process::Command;
    let name = dialoguer::Input::<String>::new()
        .with_prompt("Your Project Name")
        .interact()
        .expect("Expected an argument");
    let mut path_name = String::from("./");
    path_name.push_str(&name);
    const LINK: &str = "https://github.com/josephsawaya/aws-serverless-toolkit-template.git";
    if cfg!(windows) {
        let mut clone = String::from("git clone ");
        clone.push_str(&LINK);
        clone.push_str(" ");
        clone.push_str(&path_name);
        let mut child = Command::new("cmd")
            .args(&["/C", &clone[..]])
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("failed to execute process");

        let mut cd = String::from("cd ");
        cd.push_str(&path_name);
        child = Command::new("cmd")
            .args(&["/C", &cd[..]])
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("failed to execute process");

        std::env::set_current_dir(&path_name).expect("failed to execute process");
        child = Command::new("cmd")
            .args(&["/C", "RMDIR .\\git /S"])
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("failed to execute process");
        child = Command::new("cmd")
            .args(&["/C", "git init"])
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("failed to execute process");
    } else {
        let mut clone = String::from("git clone ");
        clone.push_str(&LINK);
        clone.push_str(" ");
        clone.push_str(&path_name);
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(clone)
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("failed to execute process");
        let mut cd = String::from("cd ");
        cd.push_str(&path_name);
        std::env::set_current_dir(&path_name).expect("failed to execute process");
        child = Command::new("sh")
            .arg("-c")
            .arg("rm -rf ./.git")
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("failed to execute process");
        child = Command::new("sh")
            .arg("-c")
            .arg("git init")
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("failed to execute process");
        child = Command::new("sh")
            .arg("-c")
            .arg("mkdir .ast_config")
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("failed to execute process");
    };
}
