use ansi_term::Style;
use chrono::{Datelike, Local, NaiveDate};
use clap::{Arg, ArgAction, Command};
use itertools::izip;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    // quite excessive integer ranges, but these are the types used by the chrono crate too
    year: i32,
    today: NaiveDate,
}

type CalResult<T> = Result<T, Box<dyn Error>>;

const MONTH_NAMES: [&str; 12] = [
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
const LINE_WIDTH: usize = 22;

pub fn get_args() -> CalResult<Config> {
    let matches = Command::new("calr")
        .version("0.1.0")
        .author("Filip Maelbrancke")
        .about("Cal command in rust")
        .arg(
            Arg::new("month")
                .value_name("MONTH")
                .short('m')
                .help("Month name or number (1-12)"),
        )
        .arg(
            Arg::new("show_current_year")
                .value_name("SHOW_YEAR")
                .short('y')
                .long("year")
                .help("Show whole current year")
                .conflicts_with_all(["month", "year"])
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("year")
                .value_name("YEAR")
                .value_parser(clap::value_parser!(i32).range(1..=9999))
                .help("Year (1-9999)"),
        )
        .get_matches();

    let mut month = matches
        .get_one("month")
        .cloned()
        .map(|v: String| parse_month(v.as_str()))
        .transpose()?;

    let mut year = matches.get_one("year").cloned();

    let today = Local::today();
    if matches.get_flag("show_current_year") {
        month = None;
        year = Some(today.year());
    } else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }

    Ok(Config {
        month,
        year: year.unwrap_or_else(|| today.year()),
        today: today.naive_local(),
    })
}

pub fn run(config: Config) -> CalResult<()> {
    //println!("{:?}", config);
    match config.month {
        Some(month) => {
            // single month
            let lines = format_month(config.year, month, true, config.today);
            println!("{}", lines.join("\n"));
        }
        None => {
            // whole year
            println!("{:>32}", config.year);
            let months: Vec<_> = (1..=12)
                .into_iter()
                .map(|month| format_month(config.year, month, false, config.today))
                .collect();

            for (i, chunk) in months.chunks(3).enumerate() {
                if let [m1, m2, m3] = chunk {
                    for lines in izip!(m1, m2, m3) {
                        // izip = iterate and combine the lines from the 3 months
                        println!("{}{}{}", lines.0, lines.1, lines.2);
                    }
                    if i < 3 {
                        println!();
                    }
                }
            }
        }
    }
    Ok(())
}

fn format_month(year: i32, month: u32, print_year: bool, today: NaiveDate) -> Vec<String> {
    let first = NaiveDate::from_ymd(year, month, 1);
    let mut days: Vec<String> = (1..first.weekday().number_from_sunday())
        .into_iter()
        .map(|_| "  ".to_string())
        .collect();

    let is_today = |day: u32| year == today.year() && month == today.month() && day == today.day();

    let last = last_day_in_month(year, month);
    days.extend((first.day()..=last.day()).into_iter().map(|num| {
        let fmt = format!("{num:>2}"); // format the day right-justified in two columns
        if is_today(num) {
            Style::new().reverse().paint(fmt).to_string() // for today use Style::reverse to highlight, otherwise use text as is
        } else {
            fmt
        }
    }));

    let month_name = MONTH_NAMES[month as usize - 1];
    let mut lines = Vec::with_capacity(8); // 8 lines of text
    lines.push(format!(
        "{:^20}  ", // 2 trailing spaces
        if print_year {
            format!("{month_name} {year}")
        } else {
            month_name.to_string()
        }
    ));

    lines.push("Su Mo Tu We Th Fr Sa  ".to_string()); // 2 trailing spaces - add days of the week

    for week in days.chunks(7) {
        // get 7 weekdays at a time
        lines.push(format!(
            "{:width$}  ", // 2 trailing spaces
            week.join(" "),
            width = LINE_WIDTH - 2
        ));
    }

    while lines.len() < 8 {
        // pad lines as needed to bring the total to 8
        lines.push(" ".repeat(LINE_WIDTH));
    }

    lines
}

fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    // start by getting first day of the next month
    let (m, y) = if month == 12 {
        (1, year + 1)
    } else {
        (month + 1, year)
    };
    // and then the preceding one is the last day of the original month
    NaiveDate::from_ymd(y, m, 1).pred()
}

fn parse_month(month: &str) -> CalResult<u32> {
    match parse_int(month) {
        Ok(num) => {
            if (1..=12).contains(&num) {
                Ok(num)
            } else {
                Err(format!("month \"{month}\" not in the range 1 through 12").into())
            }
        }
        _ => {
            let lower = &month.to_lowercase();
            let matches: Vec<_> = MONTH_NAMES
                .iter()
                .enumerate() // get index & value
                .filter_map(|(i, name)| {
                    if name.to_lowercase().starts_with(lower) {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect();
            if matches.len() == 1 {
                Ok(matches[0] as u32)
            } else {
                Err(format!("Invalid month \"{month}\"").into())
            }
        }
    }
}

fn parse_int<T: FromStr>(val: &str) -> CalResult<T> {
    // can return either i32 or u32
    val.parse() // convert string into desired return type
        .map_err(|_| format!("Invalid integer \"{val}\"").into())
}

#[cfg(test)]
mod tests {
    use crate::{format_month, last_day_in_month, parse_int, parse_month};
    use chrono::NaiveDate;

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"0\" not in the range 1 through 12"
        );

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"13\" not in the range 1 through 12"
        );

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid month \"foo\"");
    }

    #[test]
    fn test_parse_int() {
        // Parse positive int as usize
        let res = parse_int::<usize>("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);

        // Parse negative int as i32
        let res = parse_int::<i32>("-1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -1i32);

        // Fail on a string
        let res = parse_int::<i64>("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_format_month() {
        let today = NaiveDate::from_ymd(0, 1, 1);
        let leap_february = vec![
            "   February 2020      ",
            "Su Mo Tu We Th Fr Sa  ",
            "                   1  ",
            " 2  3  4  5  6  7  8  ",
            " 9 10 11 12 13 14 15  ",
            "16 17 18 19 20 21 22  ",
            "23 24 25 26 27 28 29  ",
            "                      ",
        ];
        assert_eq!(format_month(2020, 2, true, today), leap_february);

        let may = vec![
            "        May           ",
            "Su Mo Tu We Th Fr Sa  ",
            "                1  2  ",
            " 3  4  5  6  7  8  9  ",
            "10 11 12 13 14 15 16  ",
            "17 18 19 20 21 22 23  ",
            "24 25 26 27 28 29 30  ",
            "31                    ",
        ];
        assert_eq!(format_month(2020, 5, false, today), may);

        let april_hl = vec![
            "     April 2021       ",
            "Su Mo Tu We Th Fr Sa  ",
            "             1  2  3  ",
            " 4  5  6 \u{1b}[7m 7\u{1b}[0m  8  9 10  ",
            "11 12 13 14 15 16 17  ",
            "18 19 20 21 22 23 24  ",
            "25 26 27 28 29 30     ",
            "                      ",
        ];
        let today = NaiveDate::from_ymd(2021, 4, 7);
        assert_eq!(format_month(2021, 4, true, today), april_hl);
    }

    #[test]
    fn test_last_day_in_month() {
        assert_eq!(last_day_in_month(2020, 1), NaiveDate::from_ymd(2020, 1, 31));
        assert_eq!(last_day_in_month(2020, 2), NaiveDate::from_ymd(2020, 2, 29));
        assert_eq!(last_day_in_month(2020, 4), NaiveDate::from_ymd(2020, 4, 30));
    }
}
