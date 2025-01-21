use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotImplemented,
    MissingArgs,
    UnsupportedArgs,
    InvalidUtf8
}

// 实现 fmt::Display trait 来格式化错误消息
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotImplemented => {
                write!(f, "interpolation for this flavor is not implemented")
            }
            Error::MissingArgs => {
                write!(f, "not enough args when interpolating")
            }
            Error::UnsupportedArgs => {
                write!(f, "unsupported args when interpolating")
            }
            Error::InvalidUtf8 => {
                write!(f, "invalid utf8")
            }
        }
    }
}