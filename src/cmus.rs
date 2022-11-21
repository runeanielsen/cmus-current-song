use std::process::Command;
use std::str;

pub struct QueryOutput(pub String);

pub fn query() -> Option<QueryOutput> {
    let output = Command::new("cmus-remote")
        .arg("-Q")
        .output()
        .expect("cmus-remote failed to start.");

    if output.status.success() {
        Some(QueryOutput(
            str::from_utf8(&output.stdout)
                .expect("Could not convert to UTF-8.")
                .to_owned(),
        ))
    } else {
        None
    }
}
