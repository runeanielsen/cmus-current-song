use std::fmt::Display;

use crate::cmus::QueryOutput;

#[derive(Debug, PartialEq, Eq)]
pub struct Track {
    artist: String,
    title: String,
    position: u32,
    duration: u32,
}

impl Track {
    fn new(artist: String, title: String, position: u32, duration: u32) -> Track {
        Track {
            artist,
            title,
            position,
            duration,
        }
    }
}

impl Display for Track {
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

impl From<QueryOutput> for Option<Track> {
    fn from(s: QueryOutput) -> Self {
        fn field_row_value(field_rows: &[&str], field_name: &str) -> Option<String> {
            field_rows
                .iter()
                .find(|x| x.starts_with(field_name))
                .map(|x| x.replace(field_name, "").trim_start().to_owned())
        }

        let field_rows: Vec<_> = s.0.split('\n').collect();

        let status = field_row_value(&field_rows, "status").expect("Could not get status field.");

        if status == "playing" {
            Some(Track::new(
                field_row_value(&field_rows, "tag artist").unwrap_or_default(),
                field_row_value(&field_rows, "tag title").unwrap_or_default(),
                field_row_value(&field_rows, "position")
                    .expect("Could not get position field.")
                    .parse()
                    .unwrap(),
                field_row_value(&field_rows, "duration")
                    .expect("Could not get duration field.")
                    .parse()
                    .unwrap(),
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_handle_stopped_output() {
        let cmus_query_output = QueryOutput(
            "status stopped
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

        let expected = None;

        let result: Option<Track> = cmus_query_output.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn can_convert_from_playing_cmus_output_to_track() {
        let cmus_query_output = QueryOutput(
            "status playing
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

        let expected = Track {
            artist: "Amy Winehouse".to_string(),
            title: "Intro / Stronger Than Me".to_string(),
            position: 22,
            duration: 234,
        };

        let result: Option<Track> = cmus_query_output.into();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn format_track_is_being_formatted() {
        let play_info = Track {
            artist: String::from("Snorri Hallgrimsson"),
            title: String::from("…og minning þín rís hægt (Peter Gregson Rework)"),
            duration: 222,
            position: 136,
        };

        assert_eq!(
            "Snorri Hallgrimsson : …og minning þín rís hægt (Peter Gregson Rework) (02:16/03:42)",
            format!("{play_info}")
        );
    }
}
