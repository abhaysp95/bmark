use std::{fmt::Display, time::{SystemTime, UNIX_EPOCH}};

struct DateTime {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    min: u64,
    sec: u64,
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}-{:02} {:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.min, self.sec)
    }
}

fn is_leap_year(year: u64) -> bool {
    return if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        true
    } else {
        false
    }
}

/// Get `DateTime` for provided `epoch` (seconds)
/// This doesn't considers your timezone and returns `DateTime` which will be UTC in 24-hour format
fn get_datetime_for_epochs(epoch: u64) -> DateTime {
    let days_since_epoch = epoch / 86400;
    let mut cyear = 1970; // epoch year start
    let mut days_in_years = 0;
    #[allow(unused_assignments)]
    let mut days_in_cyear = 0;

    let mut days_till_now = days_since_epoch;
    while days_till_now >= 365 {
        if is_leap_year(cyear) {
            if days_till_now < 366 {
                break;
            }
            days_in_cyear = 366;
        } else {
            days_in_cyear = 365;
        }
        days_in_years += days_in_cyear;
        days_till_now -= days_in_cyear;
        cyear += 1;
    }

    let days_this_year = days_since_epoch - days_in_years;
    let days_per_month = vec![31, if is_leap_year(cyear) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut days_count = 0;
    let mut cmonth = 0;
    loop {
        days_count += days_per_month[cmonth];
        if days_count > days_this_year {
            days_count -= days_per_month[cmonth];
            break;
        }
        cmonth += 1;
    }

    let cmonth = cmonth as u64 + 1; // cmonth was zero indexed
    let seconds_today = epoch % 86400;
    let cday = days_this_year - days_count + 1;

    let chour = seconds_today / 3600;
    let cmin = (seconds_today % 3600) / 60;
    let csec = seconds_today - (chour * 3600 + cmin * 60);

    return DateTime {
        year: cyear,
        month: cmonth,
        day: cday,
        hour: chour,
        min: cmin,
        sec: csec
    }
}

pub fn validate_date(arg: &str) -> Result<String, String> {
    let today = SystemTime::now().duration_since(UNIX_EPOCH).expect("Duration before Unix Epoch");
    let epoch = today.as_secs();

    let datetime = get_datetime_for_epochs(epoch);
    println!("todays date: {}", datetime);

    Ok(arg.to_owned())
}

#[test]
fn test_datetime_from_epoch() {
    let epochs: Vec<u64> = vec![946684800, 1609459199, 253402300799, 0, 1582934400];
    let res_datetime = vec!["2000-01-01 00:00:00", "2020-12-31 23:59:59", "9999-12-31 23:59:59", "1970-01-01 00:00:00", "2020-02-29 00:00:00"];

    let mut output_datetime: Vec<String> = vec![];
    for epoch in &epochs {
        let datetime = get_datetime_for_epochs(*epoch);
        output_datetime.push(format!("{}", datetime));
    }

    for i in 0..epochs.len() {
        assert_eq!(res_datetime[i], &output_datetime[i]);
    }
}
