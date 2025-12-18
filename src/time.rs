/***
* Handle all time conversion code here.
*/

use chrono::{DateTime, Duration, Utc};
use tzfile::ArcTz; // use ArcTz as serenity uses threads

use crate::Error;

pub fn get_closest_future_time(
    time: chrono::NaiveTime,
    time_zone: tzfile::Tz,
) -> Result<DateTime<ArcTz>, Error> {
    /* This is a naive approach to finding the date, and will take on previous dates, if say the
     * current time is 23:00, and the NaiveTime passed comes before that. */
    let current_date = Utc::now();
    let working_date = Utc::now()
        .with_timezone(&ArcTz::new(time_zone))
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
