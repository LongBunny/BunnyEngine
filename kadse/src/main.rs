use std::env;
use std::path::Path;

fn main() {
    let wd = Path::new(".");
    env::set_current_dir(&wd).unwrap();
    bun::test_run();
}