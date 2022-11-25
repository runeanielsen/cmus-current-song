#![warn(clippy::all, clippy::pedantic)]

use track::Track;

mod cmus;
mod track;

fn main() {
    if let Some(output) = cmus::query() {
        match Into::<Option<Track>>::into(output) {
            Some(track) => {
                println!("{}", track);
            }
            None => {
                print!("");
            }
        }
    }
}
