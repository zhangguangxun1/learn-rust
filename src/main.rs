extern crate core;

mod get_start;
mod music;
mod postgresql;
mod tts;
mod md_parse;

fn main() {
    music::local_music::test_local_music();
}
