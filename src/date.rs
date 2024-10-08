use std::{
    error::Error,
    fmt::Display,
    panic,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{ensure, Result};
use regex::Regex;

#[derive(PartialEq)]
#[allow(dead_code)]
pub struct Datetime {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

#[allow(dead_code)]
fn get_month_name_from_index<T>(idx: T) -> String
where
    T: Into<u8>,
{
    let idx = idx.into();
    if idx == 0 || idx > 12 {
        panic!("Months range from 1 to 12")
    }
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    String::from(months[idx as usize])
}

impl Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

#[allow(dead_code)]
fn is_leap_year(year: u32) -> bool {
    return if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        true
    } else {
        false
    };
}

/// Get `DateTime` for provided `epoch` (seconds)
/// This doesn't considers your timezone and returns `DateTime` which will be UTC in 24-hour format
#[allow(dead_code)]
fn get_datetime_for_epochs(epoch: u64) -> Datetime {
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
    let days_per_month = vec![
        31,
        if is_leap_year(cyear) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

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

    return Datetime {
        year: cyear as u32,
        month: cmonth as u8,
        day: cday as u8,
        hour: chour as u8,
        minute: cmin as u8,
        second: csec as u8,
    };
}

#[allow(dead_code)]
#[derive(Debug)]
enum DatetimeError {
    ParsingError(String),
}

impl Display for DatetimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParsingError(msg) => write!(f, "DatetimeParsingError: {}", msg),
        }
    }
}

impl Error for DatetimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[allow(dead_code)]
fn parse_date(arg: &str) -> Result<Datetime> {
    let re = Regex::new(r"^(?P<year>\d{4})-(?P<mon>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<min>\d{2}):(?P<sec>\d{2})$")
        .map_err(|_| eprintln!("Failed to create regex")).unwrap();
    let cap_dt = re.captures(arg).unwrap();

    let month_with_30 = vec![4, 6, 9, 11];

    let year = cap_dt["year"].parse::<u32>().unwrap();
    ensure!(
        year >= 1970,
        DatetimeError::ParsingError(String::from("Year should be more than 1970"))
    );
    let month = cap_dt["mon"].parse::<u8>().unwrap();
    ensure!(
        month > 0 && month <= 12,
        DatetimeError::ParsingError(String::from("Month can be in range of 1 to 12"))
    );
    let day = cap_dt["day"].parse::<u8>().unwrap();
    ensure!(
        day > 0 && day <= 31,
        DatetimeError::ParsingError(String::from("Day can be in range of 1 to 31"))
    );
    ensure!(
        month_with_30.contains(&month) && day == 31,
        DatetimeError::ParsingError(String::from(format!(
            "{} can't have 31 days",
            get_month_name_from_index(month as u8)
        )))
    );
    ensure!(
        month == 2 && day > 29,
        DatetimeError::ParsingError(String::from(format!(
            "{} can't have more than 29 days",
            get_month_name_from_index(month as u8)
        )))
    );
    ensure!(
        month == 2 && day == 29 && !is_leap_year(year),
        DatetimeError::ParsingError(String::from(format!("{} is not leap year", &year)))
    );
    let hour = cap_dt["hour"].parse::<u8>().unwrap();
    ensure!(
        hour > 23,
        DatetimeError::ParsingError(String::from("Hour ranges from 0 to 23"))
    );
    let minute = cap_dt["min"].parse::<u8>().unwrap();
    ensure!(
        hour > 59,
        DatetimeError::ParsingError(String::from("Minute ranges from 0 to 59"))
    );
    let second = cap_dt["sec"].parse::<u8>().unwrap();
    ensure!(
        hour > 59,
        DatetimeError::ParsingError(String::from("Second ranges from 0 to 59"))
    );

    Ok(Datetime {
        year,
        month,
        day,
        hour,
        minute,
        second,
    })
}


#[allow(dead_code)]
pub fn get_current_datetime() -> Datetime {
    let today = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Duration befor Unix Epoch");
    return get_datetime_for_epochs(today.as_secs());
}

#[test]
fn test_datetime_from_epoch() {
    let epochs: Vec<u64> = vec![946684800, 1609459199, 253402300799, 0, 1582934400];
    let res_datetime = vec![
        "2000-01-01 00:00:00",
        "2020-12-31 23:59:59",
        "9999-12-31 23:59:59",
        "1970-01-01 00:00:00",
        "2020-02-29 00:00:00",
    ];

    let mut output_datetime: Vec<String> = vec![];
    for epoch in &epochs {
        let datetime = get_datetime_for_epochs(*epoch);
        output_datetime.push(format!("{}", datetime));
    }

    for i in 0..epochs.len() {
        assert_eq!(res_datetime[i], &output_datetime[i]);
    }
}
