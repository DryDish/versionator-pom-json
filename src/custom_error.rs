use core::fmt;

#[derive(Debug)]
pub enum CustomError {
    FileNotFound,
    IoError,
    DirectoryNotFound,
    UnableToOpenDirectory,
    UnableToOpenFile,
    TargetNotFound,
    SourceNotFound,
    BadParams,
}

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::FileNotFound => write!(f, "File Not found"),
            &Self::IoError => write!(f, "Io Error"),
            &Self::DirectoryNotFound => write!(f, "Directory Not found"),
            &Self::UnableToOpenDirectory => write!(f, "Unable to open directory"),
            &Self::UnableToOpenFile => write!(f, "Unable To Open File"),
            &Self::TargetNotFound => write!(f, "Target File Not Found"),
            &Self::SourceNotFound => write!(f, "Source File Not Found"),
            &Self::BadParams => write!(f, "Parameters Passed Not Understood"),
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
