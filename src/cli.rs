
pub struct Cli{
    pub path: std::path::PathBuf,
}

impl Cli{
    pub fn new() -> Self{
        let mut args = std::env::args_os();
        let path = args.nth(1).expect("Missing path argument");
        Self{
            path: std::path::PathBuf::from(path),
        }   
    }
}