fn main() {
    use std::env;
    let mut pattern = env::args();
    let arg_obj = types::Arg::new(&mut pattern);
    match &arg_obj.cmd()[..] {
        "new" => args::new(),
        _ => panic!("Argument not valid)"),
    }
}

mod types {
    pub struct Arg {
        cmd: String,
    }
    impl Arg {
        pub fn new(val: &mut std::env::Args) -> Arg {
            Arg {
                cmd: val.nth(1).expect("Expected an argument"),
            }
        }
        pub fn cmd(&self) -> String {
            String::from(&self.cmd)
        }
    }
}

mod args {
    pub fn new() {
        use std::process::Command;
        let name = dialoguer::Input::<String>::new()
            .with_prompt("Your Project Name:")
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
        };
    }
}
