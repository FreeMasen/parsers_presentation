extern crate duration;
use duration::{Duration, DurationPart};
extern crate combine;
use combine::{
    choice,
    char::{char, digit},
    many1,
    optional,
    Parser,
    ParseError,
    Stream,
};
pub fn parse(s: &str) -> Result<Duration, String> {
    let d = duration().parse(s).map_err(|e| format!("{}", e))?;
    Ok(d.0)
}

fn duration<I>() -> impl Parser<Input = I, Output = Duration>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        char('P'),
        optional(date_part()),
        optional(time_part()),
    ).map(|(_, d, t)| {
        let mut ret = Duration::new();
        for part in d.unwrap_or(Vec::new()).iter().chain(t.unwrap_or(Vec::new()).iter()) {
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
        ret
    })
}

fn date_part<I>() -> impl Parser<Input = I, Output = Vec<DurationPart>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        many1(value_pair(false))
    )
}

fn time_part<I>() -> impl Parser<Input = I, Output = Vec<DurationPart>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        char('T'),
        many1(value_pair(true))
    ).map(|(_, p)| p)
}

fn value_pair<I>(time: bool) -> impl Parser<Input = I, Output = DurationPart>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        float(),
        choice([
            char('Y'),
            char('M'),
            char('W'),
            char('D'),
            char('H'),
            char('S'),
        ])).map(move |(v, c): (f32, char)| {
            match c {
                'Y' => DurationPart::Years(v),
                'M' => if time {
                    DurationPart::Minutes(v)
                } else {
                    DurationPart::Months(v)
                },
                'W' => DurationPart::Weeks(v),
                'D' => DurationPart::Days(v),
                'H' => DurationPart::Hours(v),
                'S' => DurationPart::Seconds(v),
                _ => unreachable!()
            }
        })
}

fn float<I>() -> impl Parser<Input = I, Output = f32>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        many1(digit()),
        optional((
            char('.'),
            many1::<String, _>(digit())
        ))
    ).map(|(int, rem)| {
        let f = if let Some(rem) = rem {
            format!("{}.{}", int, rem.1)
        } else {
            int
        };
        f.parse().unwrap()
    })
}