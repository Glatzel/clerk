#[cfg(feature = "log")]
mod constants;
#[cfg(feature = "log")]
mod file;
#[cfg(feature = "log")]
mod term;
#[cfg(feature = "log")]
pub(crate) use constants::*;
#[cfg(feature = "log")]
pub use file::file_layer;
#[cfg(feature = "log")]
pub use term::terminal_layer;

#[cfg(feature = "log")]
pub use tracing;
#[cfg(feature = "log")]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::tracing::trace!($($arg)*);
    };
}
#[cfg(feature = "log")]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
       $crate::tracing::debug!($($arg)*);
    };
}
#[cfg(feature = "log")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
       $crate:: tracing::info!($($arg)*);
    };
}
#[cfg(feature = "log")]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::tracing::warn!($($arg)*);
    };
}
#[cfg(feature = "log")]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
       $crate:: tracing::error!($($arg)*);
    };
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {};
}
