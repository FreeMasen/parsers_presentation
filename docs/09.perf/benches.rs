#![feature(test)]

extern crate test;
extern crate duration;
extern crate random_durations;
#[cfg(feature = "nom")]
extern crate nom_duration_parser;
#[cfg(feature = "pest")]
extern crate pest_duration_parser;
#[cfg(feature = "combine")]
extern crate combine_duration_parser;
#[cfg(feature = "hand")]
extern crate hand_rolled_duration_parser;
#[macro_use]
extern crate lazy_static;

use test::{Bencher, black_box};
use duration::Duration;

lazy_static! {
    static ref DURATION: String = format!("{}", random_durations::gen_random_dur());
}

lazy_static! {
    static ref DURATIONS: Vec<String> = random_durations::gen_random_durs_text(1000);
}

#[cfg(feature = "nom")]
#[bench]
fn nom(b: &mut Bencher) {
    single(b, &nom_duration_parser::parse);
}

#[cfg(feature = "nom")]
#[bench]
fn nom_1000(b: &mut Bencher) {
    thousand(b, &nom_duration_parser::parse);
}

#[cfg(feature = "pest")]
#[bench]
fn pest(b: &mut Bencher) {
    single(b, &pest_duration_parser::parse);
}
#[cfg(feature = "pest")]
#[bench]
fn pest_1000(b: &mut Bencher) {
    thousand(b, &pest_duration_parser::parse);
}

#[cfg(feature = "combine")]
#[bench]
fn combine(b: &mut Bencher) {
    single(b, &combine_duration_parser::parse);
}
#[cfg(feature = "combine")]
#[bench]
fn combine_1000(b: &mut Bencher) {
    thousand(b, &combine_duration_parser::parse);
}

#[cfg(feature = "hand")]
#[bench]
fn hand_rolled(b: &mut Bencher) {
    single(b, &hand_rolled_duration_parser::parse);
}
#[cfg(feature = "hand")]
#[bench]
fn hand_rolled_1000(b: &mut Bencher) {
    thousand(b, &hand_rolled_duration_parser::parse);
}
fn single(b: &mut Bencher, f: &impl Fn(&str) -> Result<Duration, String>) {
    b.iter(|| {
        black_box(f(&DURATION).unwrap());
    });
}

fn thousand(b: &mut Bencher, f: &impl Fn(&str) -> Result<Duration, String>) {
    b.iter(|| {
        for s in DURATIONS.iter() {
            black_box(f(s).unwrap());
        }
    })
}