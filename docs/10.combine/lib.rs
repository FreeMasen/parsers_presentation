extern crate duration;
use duration::{Duration, DurationPart};
extern crate combine;
use combine::{
    optional,
    Parser,
    range::recognize,
    parser::{
        item::item,
        byte::digit,
        repeat::{
            skip_many,
            skip_many1
        },
        repeat::skip_count,
    },
    error::UnexpectedParse,
};
pub fn parse(s: &str) -> Result<Duration, String> {
    let value = || {
        recognize((
            skip_many1(digit()),
            optional((
                item(b'.'),
                skip_many(digit())
            )),
        ))
        .and_then(|bs: &[u8]| {
            let s = ::std::str::from_utf8(bs).map_err(|_| UnexpectedParse::Unexpected)?;
            s.parse::<f32>().map_err(|_| UnexpectedParse::Unexpected)
        })
    };
    let pair = |time: bool| {
        (
            value(),
            combine::parser::item::any()
        ).and_then(move |(v, c): (f32, u8)| {
            let part = match c {
                b'Y' => DurationPart::Years(v),
                b'M' => if time {
                    DurationPart::Minutes(v)
                } else {
                    DurationPart::Months(v)
                },
                b'W' => DurationPart::Weeks(v),
                b'D' => DurationPart::Days(v),
                b'H' => DurationPart::Hours(v),
                b'S' => DurationPart::Seconds(v),
                _ => return Err(UnexpectedParse::Unexpected)
            };
            Ok(part)
        })
    };
    let date_part = combine::count(4, pair(false)).map(|p: Vec<DurationPart>| p);
    let time_part = skip_count(1, item(b'T')).and(combine::count(3, pair(true))).map(|(_, p): (_, Vec<DurationPart>)| p);
    let mut duration = skip_count(1, item(b'P')).and(date_part).and(time_part).map(|((_, d), t): ((_, std::vec::Vec<DurationPart>), std::vec::Vec<DurationPart>)| (d, t));
    let ((date_parts, time_parts), rem): ((Vec<DurationPart>, Vec<DurationPart>), &[u8]) = duration.parse(s.as_bytes()).map_err(|e| format!("{}", e))?;
    if rem.len() > 0 {
        return Err(format!("did not parse full string provided {}", String::from_utf8_lossy(rem)));
    }
    let mut ret = Duration::new();
    for part in date_parts.iter().chain(time_parts.iter()) {
        match part {
            DurationPart::Years(v) => ret.set_years(*v),
            DurationPart::Months(v) => ret.set_months(*v),
            DurationPart::Weeks(v) => ret.set_weeks(*v),
            DurationPart::Days(v) => ret.set_days(*v),
            DurationPart::Hours(v) => ret.set_hours(*v),
            DurationPart::Minutes(v) => ret.set_minutes(*v),
            DurationPart::Seconds(v) => ret.set_seconds(*v),
        }
    }
    Ok(ret)
}