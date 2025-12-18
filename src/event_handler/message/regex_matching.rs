use chrono::{NaiveTime, format::ParseErrorKind};
use regex::Regex;
use tracing::{debug, info, warn};

use crate::structs::regex_time::TimeClue;

#[tracing::instrument(skip(haystack))]
pub fn match_simple_time(haystack: &String) -> Option<NaiveTime> {
    let regex = Regex::new(
        "\\s?(?<hr>[0-1]?[0-9]|2[0-3])(?::(?<mm>[0-5][0-9]))?(?::(?<ss>[0-5][0-9]))?\\s?(?<clue>am|pm|nn|mn)?\\s?"
    )
    .unwrap();
    let captured_string = regex.captures(haystack);

    match captured_string {
        Some(capture) => {
            info!("Found match for potential time format; attempting to parse string.");
            debug!("Message captured: \"{}\"", &haystack);

            let mut hour: u32 = capture["hr"].to_string().parse().unwrap();

            let minute: u32 = capture.name("mm").map_or(0, |x| {
                let x: String = x.as_str().to_string();
                info!("Haystack contains minutes, like %h:%m - parsing further information.");
                x.parse().unwrap()
            });
            let second = capture.name("ss").map_or(0, |x| {
                let x: String = x.as_str().to_string();
                info!("Haystack contains seconds, like %h:%m:%s - parsing further information.");
                x.parse().unwrap()
            });

            // Check if it is in 12hr time, and if so, handle am/pm/nn/mn separately.
            match (hour <= 12, capture.name("clue")) {
                (true, Some(clue)) => {
                    let capture: &str = clue.as_str();
                    handle_time_clue(capture.try_into().unwrap(), &mut hour).unwrap(); // should be safe
                }
                // Check for invalid time, like 23 pm, which wouldn't make sense.
                (false, Some(_)) => {
                    info!(
                        "Invalid time, not returning anything... (12hr time clue used in 24hr time)"
                    );
                    return None;
                }
                (true, None) => {}
                (false, None) => {}
            }

            NaiveTime::from_hms_opt(hour, minute, second)
        }
        None => None,
    }
}

#[tracing::instrument]
pub fn match_preposition_time(haystack: &String) -> Option<(NaiveTime, bool)> {
    let regex = Regex::new("(at )(?<hr>[0-1]?[0-9])(?<clue>am|pm|nn)?").unwrap();

    if let Some(capture) = regex.captures(haystack) {
        match (&capture.name("hr"), &capture.name("clue")) {
            (Some(hr), Some(clue)) => {
                let mut hr: u32 = hr.as_str().parse().unwrap();
                let clue: TimeClue = clue.as_str().try_into().unwrap();

                let _ = handle_time_clue(clue, &mut hr);

                let time = NaiveTime::from_hms_opt(hr, 0, 0).unwrap();

                Some((time, true))
            }
            (Some(hr), None) => {
                let hr: u32 = hr.as_str().parse().unwrap();
                let time = NaiveTime::from_hms_opt(hr, 0, 0).unwrap();

                Some((time, false))
            }
            (None, Some(_)) => None, // these shouldn't happen normally and we should just
            (None, None) => None,    // ignore it, we can't do anything without an hour anyways.
        }
    } else {
        None
    }
}

#[tracing::instrument(skip(time_clue, hour))]
fn handle_time_clue(time_clue: TimeClue, hour: &mut u32) -> Result<(), ParseErrorKind> {
    if *hour > 12 {
        return Err(ParseErrorKind::OutOfRange);
    };

    match time_clue {
        TimeClue::AM => {
            info!("Matched am (morning) time clue, doing nothing...")
        }
        TimeClue::NN => {
            // There's a possibility that hour is not equal to 12 at this point, but nn is not
            // often used anyways, so just ignore it.
            info!("Matched nn (noon) time clue, doing nothing...");
        }
        TimeClue::PM => {
            *hour += 12;
            info!(
                "Matched pm (afternoon/evening) time clue, adding 12 to hour given, resulting in final result: {}hrs.",
                hour
            );
        }
        TimeClue::MN => {
            *hour = 0;
            info!("Matched mn (midnight) time clue, setting hour to zero.");
        }
    };

    Ok(())
}
