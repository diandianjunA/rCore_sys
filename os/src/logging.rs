use crate::cfg_if;
#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::println!("\x1b[1;32m[INFO] {}\x1b[0m", format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::println!("\x1b[1;33m[WARN] {}\x1b[0m", format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::println!("\x1b[1;31m[ERROR] {}\x1b[0m", format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::println!("\x1b[1;34m[DEBUG] {}\x1b[0m", format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::println!("\x1b[1;30m[TRACE] {}\x1b[0m", format_args!($fmt $(, $($arg)+)?));
    };
}

cfg_if!(
    if #[cfg(feature = "info")] {
        #[macro_export]
        macro_rules! log {
            ($fmt: literal $(, $($arg: tt)+)?) => {
                $crate::info!($fmt $(, $($arg)+)?);
            };
        }
    } else if #[cfg(feature = "debug")] {
        #[macro_export]
        macro_rules! log {
            ($fmt: literal $(, $($arg: tt)+)?) => {
                $crate::debug!($fmt $(, $($arg)+)?);
            };
        }
    } else if #[cfg(feature = "trace")] {
        #[macro_export]
        macro_rules! log {
            ($fmt: literal $(, $($arg: tt)+)?) => {
                $crate::trace!($fmt $(, $($arg)+)?);
            };
        }
    } else if #[cfg(feature = "warn")] {
        #[macro_export]
        macro_rules! log {
            ($fmt: literal $(, $($arg: tt)+)?) => {
                $crate::warn!($fmt $(, $($arg)+)?);
            };
        }
    } else if #[cfg(feature = "error")] {
        #[macro_export]
        macro_rules! log {
            ($fmt: literal $(, $($arg: tt)+)?) => {
                $crate::error!($fmt $(, $($arg)+)?);
            };
        }
    } else {
        #[macro_export]
        macro_rules! log {
            ($fmt: literal $(, $($arg: tt)+)?) => {};
        }
    }
);