#![warn(clippy::all, clippy::pedantic)]

mod cmus;
mod song;

use cmus::{query_current_song, PlayInfo};

use crate::song::Song;

impl From<PlayInfo> for Song {
    fn from(play_info: PlayInfo) -> Self {
        Song::new(
            play_info
                .get_field("tag artist")
                .expect("Could not get artist field."),
            play_info
                .get_field("tag title")
                .expect("could not get title field."),
            play_info
                .get_field("position")
                .expect("could not get position field.")
                .parse()
                .unwrap(),
            play_info
                .get_field("duration")
                .expect("could not get duration field.")
                .parse()
                .unwrap(),
        )
    }
}

fn main() {
    let info = query_current_song();

    if let Some(status) = info.get_field("status") {
        if status == "playing" {
            println!("{}", Into::<Song>::into(info));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieve_tags_test() {
        let play_info = PlayInfo::new(
            &["status playing",
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
              "set vol_right 100"]);

        let expected = Song {
            artist: String::from("Snorri Hallgrimsson"),
            title: String::from("…og minning þín rís hægt (Peter Gregson Rework)"),
            duration: 222,
            position: 136,
        };

        assert_eq!(expected, Into::into(play_info));
    }
}
