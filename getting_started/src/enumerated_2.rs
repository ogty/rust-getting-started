#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

impl TimeUnit {
    fn to_seconds(&self) -> u64 {
        match self {
            TimeUnit::Seconds => 1,
            TimeUnit::Minutes => 60,
            TimeUnit::Hours => 60 * 60,
            TimeUnit::Days => 60 * 60 * 24,
            TimeUnit::Weeks => 60 * 60 * 24 * 7,
            TimeUnit::Months => 60 * 60 * 24 * 30,
            TimeUnit::Years => 60 * 60 * 24 * 365,
        }
    }

    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Weeks => "weeks",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
}

fn main() {
    let seconds = TimeUnit::Seconds;
    let minutes = TimeUnit::Minutes;
    let hours = TimeUnit::Hours;
    let days = TimeUnit::Days;
    let weeks = TimeUnit::Weeks;
    let months = TimeUnit::Months;
    let years = TimeUnit::Years;

    println!("{}", seconds.to_seconds());
    println!("{}", minutes.to_seconds());
    println!("{}", hours.to_seconds());
    println!("{}", days.to_seconds());
    println!("{}", weeks.to_seconds());
    println!("{}", months.to_seconds());
    println!("{}", years.to_seconds());

    println!("{}", seconds.plural());
    println!("{}", minutes.plural());
    println!("{}", hours.plural());
    println!("{}", days.plural());
    println!("{}", weeks.plural());
    println!("{}", months.plural());
    println!("{}", years.plural());
}
