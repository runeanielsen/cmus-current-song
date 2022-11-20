#![warn(clippy::all, clippy::pedantic)]

mod cmus;

fn main() {
    if let Some(info) = cmus::query_play_info() {
        println!("{}", info);
    }
}
