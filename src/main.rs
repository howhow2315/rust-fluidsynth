extern crate fluidsynth;
extern crate rand;
extern crate time;

use fluidsynth::*;
use rand::RngExt;
use time::Duration;
use std::thread;

const SF2_PATH: &str = "/home/howhow/development/music/SF2/SGM-V2.01.sf2"; //"/path/to/soundfont.sf2";

fn main() {
    let mut settings = settings::Settings::new();
    let mut syn = synth::Synth::new(&mut settings);
    let _adriver = audio::AudioDriver::new(&mut settings, &mut syn);
    syn.sfload(SF2_PATH, 1);

    let interval = Duration::milliseconds(1000);
    let mut rng = rand::rng();

    for _x in 0..12 {
        let num: i32 = rng.random_range(1..=12);
        let key = 60 + num;
        syn.noteon(0, key, 80);
        thread::sleep(interval.unsigned_abs());
        syn.noteoff(0, key);
    }
}