fn main() {
    use std::env;
    let mut pattern = env::args();
    let arg_obj = types::Arg::new(&mut pattern);
    match &arg_obj.cmd()[1][..] {
        "new" => args::new::new(),
        "add" => args::add::add(&arg_obj.cmd()[2]),
        "clear" => args::clear::clear(),
        _ => panic!("Argument not valid)"),
    }
}

pub mod args;
pub mod helper;
pub mod types;
