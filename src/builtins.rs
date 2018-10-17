use std::env;
use std::path::Path;
use exec::ExitStatus;

pub fn cd_command(argv: &Vec<String>) -> ExitStatus {
    trace!("cd: {:?}", argv);
    let dir = match argv.get(1) {
        Some(dir) => {
            if dir.starts_with("/") {
                dir.clone()
            } else {
                let current_dir = env::current_dir().expect("failed to getcwd()");
                Path::new(&current_dir).join(dir.clone()).to_string_lossy().into_owned()
            }
        }
        None => {
            if let Some(home_dir) = env::home_dir() {
                home_dir.to_string_lossy().into_owned()
            } else {
                String::from("/")
            }
        }
    };

    if env::set_current_dir(&dir).is_ok() {
        0
    } else {
        error!("failed to cd into {}", dir);
        1
    }
}