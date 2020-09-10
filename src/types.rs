pub struct Arg {
    cmd: std::vec::Vec<String>,
}
impl Arg {
    pub fn new(val: &mut std::env::Args) -> Arg {
        Arg { cmd: val.collect() }
    }
    pub fn cmd(&self) -> &std::vec::Vec<String> {
        &self.cmd
    }
}
