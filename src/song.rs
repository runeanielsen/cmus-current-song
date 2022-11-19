use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Song {
    pub artist: String,
    pub title: String,
    pub position: u32,
    pub duration: u32,
}

impl Song {
    pub fn new(artist: String, title: String, position: u32, duration: u32) -> Song {
        Song {
            artist,
            title,
            position,
            duration,
        }
    }
}

impl Display for Song {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_song_test() {
        let song = Song {
            artist: String::from("Snorri Hallgrimsson"),
            title: String::from("…og minning þín rís hægt (Peter Gregson Rework)"),
            duration: 222,
            position: 136,
        };

        assert_eq!(
            "Snorri Hallgrimsson : …og minning þín rís hægt (Peter Gregson Rework) (02:16/03:42)",
            format!("{}", song)
        );
    }
}
