#![warn(clippy::all, clippy::pedantic)]

use std::{process::Command, str};

#[derive(Debug, PartialEq)]
struct Song {
    status: String,
    artist: String,
    title: String,
    position: u32,
    duration: u32,
}

fn format_time(t: u32) -> String {
    format!("{:0>2}:{:0>2}", (t / 60), (t % 60))
}

fn format_song(song: &Song) -> String {
    format!(
        "{} : {} ({}/{})",
        song.artist,
        song.title,
        format_time(song.position),
        format_time(song.duration)
    )
}

fn query_song_information() -> String {
    let output = Command::new("cmus-remote")
        .arg("-Q")
        .current_dir("/bin")
        .output()
        .expect("ls command failed to start");

    str::from_utf8(&output.stdout)
        .expect("Could not convert to UTF-8.")
        .to_owned()
}

fn get_song_field(fields: &[&str], field_name: &str) -> Option<String> {
    fields
        .iter()
        .find(|x| x.contains(field_name))
        .map(|x| x.replace(field_name, "").trim_start().to_owned())
}

fn make_song(information: &str) -> Song {
    let fields: Vec<&str> = information.split('\n').collect();
    Song {
        status: get_song_field(&fields, "status").expect("Could not get status field."),
        artist: get_song_field(&fields, "tag artist").expect("Could not get artist field."),
        title: get_song_field(&fields, "tag title").expect("could not get title field."),
        position: get_song_field(&fields, "position")
            .expect("could not get position field.")
            .parse()
            .unwrap(),
        duration: get_song_field(&fields, "duration")
            .expect("could not get duration field.")
            .parse()
            .unwrap(),
    }
}

fn main() {
    let information = query_song_information();
    let song = make_song(&information);
    if song.status == "playing" {
        println!("{}", format_song(&song));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_song_test() {
        let song = Song {
            status: String::from("Playing"),
            artist: String::from("Snorri Hallgrimsson"),
            title: String::from("…og minning þín rís hægt (Peter Gregson Rework)"),
            duration: 222,
            position: 136,
        };

        assert_eq!(
            "Snorri Hallgrimsson : …og minning þín rís hægt (Peter Gregson Rework) (02:16/03:42)",
            format_song(&song)
        );
    }

    #[test]
    fn format_time_test() {
        let assertions = vec![
            ("02:16", 136),
            ("03:42", 222),
            ("00:00", 0),
            ("00:10", 10),
            ("00:30", 30),
            ("01:00", 60),
            ("01:03", 63),
            ("11:00", 660),
            ("11:10", 670),
        ];

        for assert in assertions {
            assert_eq!(assert.0, format_time(assert.1));
        }
    }

    #[test]
    fn get_song_field_test() {
        let fields = vec!(
            "status playing",
            "file /home/notation/nas/music/Snorri_Hallgrimsson-Orbit_Reworked/01-02-Peter_Gregson-og_minning_bi_n_ri_s_haegt-SMR.flac",
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
            assert_eq!(assertion.0, get_song_field(&fields, assertion.1).expect(""));
        }
    }

    #[test]
    fn retrieve_tags_test() {
        let text = "status playing
                    file /home/notation/nas/music/Snorri_Hallgrimsson-Orbit_Reworked/01-02-Peter_Gregson-og_minning_bi_n_ri_s_haegt-SMR.flac
                    duration 222
                    position 136
                    tag album Orbit Reworked
                    tag title …og minning þín rís hægt (Peter Gregson Rework)
                    tag tracknumber 2
                    tag discnumber 1
                    tag date 2018
                    tag genre Électronique
                    tag albumartist Snorri Hallgrimsson
                    tag artist Snorri Hallgrimsson
                    set aaa_mode artist
                    set continue true
                    set play_library true
                    set play_sorted false
                    set replaygain disabled
                    set replaygain_limit true
                    set replaygain_preamp 0.000000
                    set repeat false
                    set repeat_current false
                    set shuffle off
                    set softvol false
                    set vol_left 100
                    set vol_right 100";

        let expected = Song {
            artist: String::from("Snorri Hallgrimsson"),
            title: String::from("…og minning þín rís hægt (Peter Gregson Rework)"),
            status: String::from("playing"),
            duration: 222,
            position: 136,
        };

        assert_eq!(expected, make_song(text));
    }
}
