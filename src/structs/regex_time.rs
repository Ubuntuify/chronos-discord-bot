use chrono::format::ParseErrorKind;

#[derive(Debug)]
pub enum TimeClue {
    AM, // morning
    PM, // afternoon or evening
    MN, // midnight
    NN, // noon
}

impl TryFrom<&str> for TimeClue {
    type Error = ParseErrorKind;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &(value.to_lowercase())[..] {
            // case insensitive
            "am" => Ok(Self::AM),
            "pm" => Ok(Self::PM),
            "mn" => Ok(Self::MN),
            "nn" => Ok(Self::NN),
            _ => Err(ParseErrorKind::Invalid),
        }
    }
}
