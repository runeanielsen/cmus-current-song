use std::fmt::Display;
use std::process::Command;
use std::str;

#[derive(Debug, PartialEq, Eq)]
pub struct PlayInfo {
    artist: String,
    title: String,
    position: u32,
    duration: u32,
}

impl Display for PlayInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn format_time(t: u32) -> String {
            format!("{:0>2}:{:0>2}", (t / 60), (t % 60))
        }

        write!(
            f,
            "{} : {} ({}/{})",
            self.artist,
            self.title,
            format_time(self.position),
            format_time(self.duration)
        )
    }
}

impl PlayInfo {
    fn new(fields: &[&str]) -> PlayInfo {
        PlayInfo {
            artist: get_field_value(fields, "tag artist").expect("Could not get artist field."),
            title: get_field_value(fields, "tag title").expect("could not get title field."),
            position: get_field_value(fields, "position")
                .expect("could not get position field.")
                .parse()
                .unwrap(),
            duration: get_field_value(fields, "duration")
                .expect("could not get duration field.")
                .parse()
                .unwrap(),
        }
    }
}

fn get_field_value(fields: &[&str], field_name: &str) -> Option<String> {
    fields
        .iter()
        .find(|x| x.contains(field_name))
        .map(|x| x.replace(field_name, "").trim_start().to_owned())
}

pub fn query_play_info() -> Option<PlayInfo> {
    let output = Command::new("cmus-remote")
        .arg("-Q")
        .current_dir("/bin")
        .output()
        .expect("cmus-remote failed to start");

    if output.status.success() {
        let fields: Vec<_> = str::from_utf8(&output.stdout)
            .expect("Could not convert to UTF-8.")
            .split('\n')
            .collect();

        Some(PlayInfo::new(&fields))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_new_play_info_test() {
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

        let expected = PlayInfo {
            artist: "Snorri Hallgrimsson".to_string(),
            title: "…og minning þín rís hægt (Peter Gregson Rework)".to_string(),
            position: 136,
            duration: 222,
        };

        let result = PlayInfo::new(&fields);

        assert_eq!(expected, result);
    }

    #[test]
    fn get_field_for_on_field_name() {
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
            assert_eq!(
                assertion.0,
                get_field_value(&fields, assertion.1).expect("")
            );
        }
    }

    #[test]
    fn format_song_is_displayed_correctly() {
        let play_info = PlayInfo {
            artist: String::from("Snorri Hallgrimsson"),
            title: String::from("…og minning þín rís hægt (Peter Gregson Rework)"),
            duration: 222,
            position: 136,
        };

        assert_eq!(
            "Snorri Hallgrimsson : …og minning þín rís hægt (Peter Gregson Rework) (02:16/03:42)",
            format!("{}", play_info)
        );
    }
}
