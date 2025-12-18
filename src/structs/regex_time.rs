use chrono::format::ParseErrorKind;

#[derive(Debug)]
pub enum TimeClue {
    AM,
    PM,
    MN,
    NN,
}

impl TryFrom<&str> for TimeClue {
    type Error = ParseErrorKind;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &(value.to_lowercase())[..] {
            "am" => Ok(Self::AM),
            "pm" => Ok(Self::PM),
            "mn" => Ok(Self::MN),
            "nn" => Ok(Self::NN),
            _ => Err(ParseErrorKind::Invalid),
        }
    }
}
