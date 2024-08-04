use std::time::{SystemTime, UNIX_EPOCH};

struct DateTime {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    min: u64,
    sec: u64,
}

fn is_leap_year(year: u64) -> bool {
    return if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        true
    } else {
        false
    }
}

fn get_todays_date() -> DateTime {
    let today = SystemTime::now().duration_since(UNIX_EPOCH).expect("Duration before Unix Epoch");
    let total_secs = today.as_secs();

    let days_since_epoch = total_secs / 86400;
    let mut cyear = 1970; // epoch year start
    let mut days_in_years = 0;
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
    let seconds_today = total_secs % 86400;
    let cday = days_this_year - days_count + if seconds_today != 0 { 1 } else { 0 };

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
    _ = get_todays_date();

    Ok(arg.to_owned())
}
