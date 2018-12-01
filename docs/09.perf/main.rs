extern crate duration;
extern crate random_durations;
#[cfg(all(feature = "nom", not(feature = "bench")))]
extern crate nom_duration_parser as parser;
#[cfg(all(feature = "combine", not(feature = "bench")))]
extern crate combine_duration_parser as parser;
#[cfg(all(feature = "pest", not(feature = "bench")))]
extern crate pest_duration_parser as parser;
#[cfg(all(feature = "hand", not(feature = "bench")))]
extern crate hand_rolled_duration_parser as parser;

#[cfg(any(feature = "bench", not(any(feature = "nom", feature = "combine", feature = "pest", feature = "hand"))))]
fn main() {
    println!("{}", random_durations::gen_random_durs_text(get_count()).join("\n"));
}

#[cfg(
    all(
        any(feature = "nom", feature = "combine", feature = "pest", feature = "hand"),
        not(feature = "bench")
        )
)]
fn main() {
    for d in random_durations::gen_random_durs(get_count()) {
        let s = format!("{}", d);
        let p = parser::parse(&s).unwrap();
        assert_eq!(d, p);
        println!("duration:{}\nparsed to\n{}", d, p.human_readable());
    }
}

fn get_count() -> usize {
    for arg in ::std::env::args() {
        match arg.parse() {
            Ok(u) => return u,
            Err(_) => ()
        }
    }
    return 1000
}