use core::fmt;

#[derive(Debug)]
pub enum CustomError {
    FileNotFound,
    IoError,
    BadParams,
    VersionNotFound,
    HelpPrinted,
}

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::FileNotFound => write!(f, "File Not found"),
            &Self::IoError => write!(f, "Io Error"),
            &Self::BadParams => write!(f, "Parameters Passed Not Understood"),
            &Self::VersionNotFound => write!(f, "String Not Found"),
            &Self::HelpPrinted => write!(f, ""),
        }
    }
}
impl From<std::io::Error> for CustomError {
    fn from(_: std::io::Error) -> Self {
        Self::IoError
    }
}

impl From<walkdir::Error> for CustomError {
    fn from(_: walkdir::Error) -> Self {
        Self::IoError
    }
}
