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
    pub fn new() -> () {
        let name = dialoguer::Input::<String>::new()
            .with_prompt("Your Project Name:")
            .interact()
            .expect("Expected an argument");
        let mut path_name = String::from("./");
        path_name.push_str(&name);
        let list = vec!["@bgpc/aws-serverless-toolkit", "typescript", "ts-node"];
        use cmd_lib::run_cmd;
        if run_cmd! {
            mkdir $path_name;
            cd $path_name;
            yarn init -y || npm init -y;
        }
        .is_err()
        {
            panic!("Error! Make sure you have npm or yarn installed.");
        }
        for arg in &list {
            println!("{}", path_name);
            if run_cmd! {
                cd $path_name;
                yarn add -D $arg || npm add -d $arg;
            }
            .is_err()
            {
                panic!("Error! Make sure you have npm or yarn installed.");
            }
        }
        if run_cmd! {
            cd $path_name;
            yarn add -D npm-add-script || npm add -d npm-add-script;
            npx npm-add-script -k "configure:dev" -v "NODE_ENV=development ts-node -r dotenv/config src/configure.ts";
        }
        .is_err()
        {
            panic!("Error! Make sure you have npm or yarn installed.");
        }
    }
}
