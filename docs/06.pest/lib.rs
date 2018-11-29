extern crate duration;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use duration::Duration;
use pest::Parser;

#[derive(Parser)]
#[grammar = "duration.pest"]
pub struct DurationParser;

pub fn parse(s: &str) -> Result<Duration, String> {
    let duration = DurationParser::parse(Rule::Duration, s)
                .map_err(|e| format!("{}", e))?
                .next()
                .unwrap();
    let ret = assemble_parts(duration)?;
    Ok(ret)
}
use pest::iterators::Pair;
fn assemble_parts(pair: pest::iterators::Pair<Rule>) -> Result<Duration, String> {
    let mut ret = Duration::new();
    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::date_section => {
                assemble_part(&mut ret, part, false)?;
            },
            Rule::time_section => {
                assemble_part(&mut ret, part, true)?;
            },
            _ => unreachable!()
        }
    }
    Ok(ret)
}

fn assemble_part(d: &mut Duration, pair: pest::iterators::Pair<Rule>, time: bool) -> Result<(), String> {
    for ref part in pair.into_inner() {
        update_duration(d, part, time)?;
    }
    Ok(())
}

fn update_duration(d: &mut Duration, pair: &Pair<Rule>, time: bool) -> Result<(), String> {
    let f = get_float(pair)?;
    match pair.as_rule() {
        Rule::year => {
            d.set_years(f);
        },
        Rule::minute_or_month => {
            if time {
                //minute
                d.set_minutes(f);
            } else {
                //month
                d.set_months(f);
            }
        },
        Rule::week => {
            d.set_weeks(f);
        },
        Rule::day => {
            d.set_days(f);
        }
        Rule::hour => {
            d.set_hours(f);
        }
        Rule::second => {
            d.set_seconds(f);
        },
        _ => unreachable!()
    }
    Ok(())
}

fn get_float(pair: &Pair<Rule>) -> Result<f32, String> {
    let s = pair.as_str();
    let s = &s[..s.len() - 1];
    s.parse().map_err(|e| format!("error parsing float: {:?} {}", s, e))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_of_each() {
        parse("P1Y1M1W1DT1H1M1.1S").unwrap();
    }
}