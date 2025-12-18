/***
* Handle all time conversion code here.
*/

use chrono::{DateTime, Duration, Timelike, Utc, format::ParseErrorKind};
use chrono_tz::Tz;

use crate::Error;

pub fn get_closest_future_time(
    time: chrono::NaiveTime,
    time_zone: Tz,
) -> Result<DateTime<Tz>, Error> {
    /* This is a naive approach to finding the date, and will take on previous dates, if say the
     * current time is 23:00, and the NaiveTime passed comes before that. */
    let current_date = Utc::now();
    let working_date = Utc::now()
        .with_timezone(&time_zone)
        .with_time(time) // set time *after*, as with_timezone will
        // change the time as well.
        .unwrap();

    // Simple compare to see whether to satisfy "future"
    if current_date.lt(&working_date) {
        Ok(working_date)
    } else {
        Ok(working_date + Duration::days(1))
    }
}

pub fn get_closest_future_time_12hr(
    time: chrono::NaiveTime,
    time_zone: Tz,
) -> Result<DateTime<Tz>, ParseErrorKind> {
    if !time.hour() <= 12 {
        return Err(ParseErrorKind::OutOfRange);
    }

    let now = Utc::now();
    let mut working_date = Utc::now()
        .with_timezone(&time_zone)
        .with_time(time)
        .unwrap();

    let mut counter = 0; // (set a counter to make sure that this ends eventually)
    while now.lt(&working_date) | (counter >= 4) {
        counter += 1;
        working_date += Duration::hours(12);
    }

    Ok(working_date)
}
