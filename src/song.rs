use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Song {
    pub artist: String,
    pub title: String,
    pub position: u32,
    pub duration: u32,
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
