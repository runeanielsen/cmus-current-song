#![warn(clippy::all, clippy::pedantic)]

use track::Track;

mod cmus;
mod track;

fn main() {
    if let Some(output) = cmus::query() {
        if let Some(track) = Into::<Option<Track>>::into(output) {
            println!("{}", track);
        }
    }
}
