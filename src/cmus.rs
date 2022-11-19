use std::process::Command;
use std::str;

pub struct PlayInfo(Vec<String>);

impl PlayInfo {
    pub fn new(fields: &[&str]) -> PlayInfo {
        PlayInfo(
            fields
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        )
    }

    pub fn get_field(&self, field_name: &str) -> Option<String> {
        self.0
            .iter()
            .find(|x| x.contains(field_name))
            .map(|x| x.replace(field_name, "").trim_start().to_owned())
    }
}

pub fn query_current_song() -> PlayInfo {
    let output = Command::new("cmus-remote")
        .arg("-Q")
        .current_dir("/bin")
        .output()
        .expect("ls command failed to start");

    let fields: Vec<_> = str::from_utf8(&output.stdout)
        .expect("Could not convert to UTF-8.")
        .split('\n')
        .collect();

    PlayInfo::new(&fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_field_test() {
        let fields = vec!(
            "status playing",
            "file /home/user/music/Snorri_Hallgrimsson-Orbit_Reworked/01-02-Peter_Gregson-og_minning_bi_n_ri_s_haegt-SMR.flac",
            "duration 222",
            "position 136",
            "tag album Orbit Reworked",
            "tag title …og minning þín rís hægt (Peter Gregson Rework)",
            "tag tracknumber 2",
            "tag discnumber 1",
            "tag date 2018",
            "tag genre Électronique",
            "tag albumartist Snorri Hallgrimsson",
            "tag artist Snorri Hallgrimsson",
            "set aaa_mode artist",
            "set continue true",
            "set play_library true",
            "set play_sorted false",
            "set replaygain disabled",
            "set replaygain_limit true",
            "set replaygain_preamp 0.000000",
            "set repeat false",
            "set repeat_current false",
            "set shuffle off",
            "set softvol false",
            "set vol_left 100",
            "set vol_right 100");

        let play_info = PlayInfo::new(&fields);

        let assertions = vec![
            ("playing", "status"),
            ("Snorri Hallgrimsson", "tag artist"),
            (
                "…og minning þín rís hægt (Peter Gregson Rework)",
                "tag title",
            ),
            ("222", "duration"),
            ("136", "position"),
        ];

        for assertion in assertions {
            assert_eq!(assertion.0, play_info.get_field(assertion.1).expect(""));
        }
    }
}
