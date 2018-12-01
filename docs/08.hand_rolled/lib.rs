extern crate duration;
use duration::{Duration, DurationPart};


pub fn parse(s: &str) -> Result<Duration, String> {
    if &s[0..1] != "P" {
        return Err(format!("All durations must start with a P: {:?}", s));
    }
    let s = &s[1..];
    let mut parts = s.split('T');
    let mut found_one = false;
    let mut ret = Duration::new();
    if let Some(date_part) = parts.next() {
        if date_part != "" {
            found_one = true;
            for part in parse_parts(date_part, false)? {
                update_duration(&mut ret, &part);
            }
        }
    }
    if let Some(time_part) = parts.next() {
        if time_part != "" {
            found_one = true;
            for part in parse_parts(time_part, true)? {
                update_duration(&mut ret, &part);
            }
        }
    }
    if !found_one {
        return Err(format!("duration contains no information: {:?}", s));
    }

    Ok(ret)
}

fn parse_parts(s: &str, is_time: bool) -> Result<Vec<DurationPart>, String> {
    let idxs = s.char_indices().filter_map(|(i, c)| {
        if c == 'Y'
        || c == 'M'
        || c == 'W'
        || c == 'D'
        || c == 'H'
        || c == 'M'
        || c == 'S' {
            Some(i)
        } else {
            None
        }
    });
    let mut ret = Vec::with_capacity(4);
    let mut start_idx = 0;
    for idx in idxs {
        let float: f32 = s[start_idx..idx].parse().map_err(|e| format!("{}", e))?;
        let tag = &s[idx..idx+1];
        let part = match tag {
            "Y" => DurationPart::Years(float),
            "M" => if is_time {
                DurationPart::Minutes(float)
            } else {
                DurationPart::Months(float)
            },
            "W" => DurationPart::Weeks(float),
            "D" => DurationPart::Days(float),
            "H" => DurationPart::Hours(float),
            "S" => DurationPart::Seconds(float),
            _ => return Err(format!("Invalid unit tag pair at {} in {:?}", idx, s)),
        };
        ret.push(part);
        start_idx = idx + 1;
    }
    Ok(ret)
}

fn update_duration(d: &mut Duration, part: &DurationPart) {
    match part {
        DurationPart::Years(v) => d.set_years(*v),
        DurationPart::Months(v) => d.set_months(*v),
        DurationPart::Weeks(v) => d.set_weeks(*v),
        DurationPart::Days(v) => d.set_days(*v),
        DurationPart::Hours(v) => d.set_hours(*v),
        DurationPart::Minutes(v) => d.set_minutes(*v),
        DurationPart::Seconds(v) => d.set_seconds(*v),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn all() {
        let d = "P1Y1M1W1DT1H1M1.1S";
        let p = parse(d).unwrap();
        assert_eq!(d, &format!("{}", p));
    }
    #[test]
    fn time_only() {
        let d = "PT1H1M1.1S";
        let p = parse(d).unwrap();
        assert_eq!(d, &format!("{}", p));
    }
    #[test]
    fn date_only() {
        let d = "P1Y1M1W1D";
        let p = parse(d).unwrap();
        assert_eq!(d, &format!("{}", p));
    }
}