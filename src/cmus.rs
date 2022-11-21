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

impl PlayInfo {
    fn new(artist: String, title: String, position: u32, duration: u32) -> PlayInfo {
        PlayInfo {
            artist,
            title,
            position,
            duration,
        }
    }
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

struct CmusQueryOutput(String);

impl From<CmusQueryOutput> for PlayInfo {
    fn from(s: CmusQueryOutput) -> Self {
        fn field_value(fields: &[&str], field_name: &str) -> Option<String> {
            fields
                .iter()
                .find(|x| x.contains(field_name))
                .map(|x| x.replace(field_name, "").trim_start().to_owned())
        }

        let fields: Vec<_> = s.0.split('\n').collect();

        PlayInfo::new(
            field_value(&fields, "tag artist").expect("Could not get artist field."),
            field_value(&fields, "tag title").expect("could not get title field."),
            field_value(&fields, "position")
                .expect("could not get position field.")
                .parse()
                .unwrap(),
            field_value(&fields, "duration")
                .expect("could not get duration field.")
                .parse()
                .unwrap(),
        )
    }
}

pub fn query_play_info() -> Option<PlayInfo> {
    let output = Command::new("cmus-remote")
        .arg("-Q")
        .output()
        .expect("cmus-remote failed to start.");

    if output.status.success() {
        Some(
            CmusQueryOutput(
                str::from_utf8(&output.stdout)
                    .expect("Could not convert to UTF-8.")
                    .to_owned(),
            )
            .into(),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_from_cmus_query_output_to_play_info() {
        let cmus_query_output = CmusQueryOutput(
            "status stopped
file /music/Amy_Winehouse-Frank/01-01-Amy_Winehouse-Intro_Stronger_Than_Me-SMR.flac
duration 234
position 22
tag album Frank
tag title Intro / Stronger Than Me
tag tracknumber 1
tag discnumber 1
tag date 2003
tag genre Soul / Funk / R&B
tag albumartist Amy Winehouse
tag artist Amy Winehouse
set aaa_mode artist
set continue true
set play_library true
set play_sorted false
set replaygain disabled
set replaygain_limit true
set replaygain_preamp 0.000000
set repeat false
set repeat_current false
set shuffle albums
set softvol false
set vol_left 100
set vol_right 100
"
            .to_string(),
        );

        let expected = PlayInfo {
            artist: "Amy Winehouse".to_string(),
            title: "Intro / Stronger Than Me".to_string(),
            position: 22,
            duration: 234,
        };

        assert_eq!(expected, cmus_query_output.into());
    }

    #[test]
    fn format_play_info_is_being_formatted() {
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
