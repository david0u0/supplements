use crate::parsed_flag::Error as ParsedFlagError;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ParsedFlag(ParsedFlagError),
    BoolFlagEqualsValue(String),
    FlagNoValue(&'static str),
    FlagNotFound(String),
    NoPossibleCompletion,
    UnexpectedArg(String), // try to provide arg where there isn't any
    ArgsTooShort,
}

impl From<ParsedFlagError> for Error {
    fn from(value: ParsedFlagError) -> Self {
        Error::ParsedFlag(value)
    }
}

#[cfg(any(feature = "clap-3", feature = "clap-4"))]
#[derive(Debug)]
pub enum GenerateError {
    UnprocessedConfigObj(Vec<Vec<String>>), // TODO: test this
    IO(std::io::Error),
}
impl From<std::io::Error> for GenerateError {
    fn from(value: std::io::Error) -> Self {
        GenerateError::IO(value)
    }
}
