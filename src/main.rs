#![warn(clippy::all, clippy::pedantic)]

use track::Track;

mod cmus;
mod track;

fn main() {
    if let Some(output) = cmus::query() {
        println!("{}", Into::<Track>::into(output));
    }
}
