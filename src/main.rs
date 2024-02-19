use chrono::{DateTime, Datelike, Local, TimeZone};

fn main() {
    let now = Local::now();

    println!("Progress of this Year: {:.6}%", calc_poy(now));
}

fn calc_poy(time: DateTime<Local>) -> f64 {
    let year_start
        = Local.with_ymd_and_hms(time.year(), 1, 1, 0, 0, 0).unwrap();

    let next_year_start
        = Local.with_ymd_and_hms(time.year() + 1, 1, 1, 0, 0, 0).unwrap();

    let total_duration = next_year_start - year_start;
    let elapsed_duration = time - year_start;

    (elapsed_duration.num_seconds() as f64 / total_duration.num_seconds() as f64) * 100.0
}

#[test]
fn zero_percent_at_year_start() {
    let time
        = Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(0.0, calc_poy(time));
}

#[test]
fn not_hundred_percent_at_year_end() {
    let time
        = Local.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();
    assert_ne!(100.0, calc_poy(time));
}
