use std::time::{SystemTime, UNIX_EPOCH};

struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
}

fn is_leap_year(year: u32) -> bool {
    return if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        true
    } else {
        false
    }
}

fn get_todays_date() -> DateTime {

    let today = SystemTime::now().duration_since(UNIX_EPOCH).expect("Duration before Unix Epoch");
    let total_sec = today.as_secs();

    let days_since_epoch = total_sec / 86400;
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

    println!("year: {}, days left: {}\n", cyear, days_since_epoch - days_in_years);

    return DateTime {
        year: cyear,
        month: 0,
        day: 0,
        hour: 0,
        min: 0,
        sec: 0
    }
}

pub fn validate_date(arg: &str) -> Result<String, String> {
    _ = get_todays_date();

    Ok(arg.to_owned())
}
