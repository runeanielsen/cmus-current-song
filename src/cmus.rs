use std::process::Command;
use std::str;

pub fn query_current_song() -> String {
    let output = Command::new("cmus-remote")
        .arg("-Q")
        .current_dir("/bin")
        .output()
        .expect("ls command failed to start");

    str::from_utf8(&output.stdout)
        .expect("Could not convert to UTF-8.")
        .to_owned()
}
