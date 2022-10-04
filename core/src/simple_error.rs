use std::{error::Error, fmt, fmt::Write};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
#[error("{0}")]
pub struct SimpleError(String);

impl SimpleError {
    pub fn new<S: ToString>(s: S) -> Self {
        SimpleError(s.to_string())
    }
}

pub trait IntoErrorReport {
    fn into_report(self) -> String;
}

impl<'a, E> IntoErrorReport for &'a E
where
    E: Error,
{
    fn into_report(self) -> String {
        let try_fun = move || -> Result<String, fmt::Error> {
            let mut output = String::new();
            let o = &mut output;
            writeln!(o, "[ERROR] {}", self)?;
            if let Some(cause) = self.source() {
                writeln!(o)?;
                writeln!(o, "Caused by:")?;
                let mut cause = Some(cause);
                let mut i = 0;
                while let Some(e) = cause {
                    writeln!(o, "   {}: {}", i, e)?;
                    cause = e.source();
                    i += 1;
                }
            }
            Ok(output)
        };

        try_fun().expect("a formatting trait implementation returned an error")
    }
}

/// Helper to report err and panic.
pub trait UnwrapReport<T> {
    fn unwrap_report(self) -> T
    where
        Self: Sized;
}

impl<T, E> UnwrapReport<T> for Result<T, E>
where
    E: Error,
{
    fn unwrap_report(self) -> T {
        match self {
            Ok(ok) => ok,
            Err(err) => {
                let report = err.into_report();
                log::error!("{}", report);
                panic!("Called `unwrap_report` on an `Err` value");
            }
        }
    }
}

impl<T> UnwrapReport<T> for Option<T> {
    fn unwrap_report(self) -> T {
        match self {
            Some(ok) => ok,
            None => {
                panic!("Called `unwrap_report` on a None value");
            }
        }
    }
}

/// Helper to unwrap Result that does not implement Debug
pub trait UnwrapNoDebug<T> {
    fn unwrap_no_debug(self) -> T
    where
        Self: Sized;
}

impl<T, E> UnwrapNoDebug<T> for Result<T, E> {
    fn unwrap_no_debug(self) -> T {
        match self {
            Ok(ok) => ok,
            Err(_) => {
                panic!("Called `unwrap_no_debug` on an `Err` value");
            }
        }
    }
}
