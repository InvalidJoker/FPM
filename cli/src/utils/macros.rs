// src/macros
// ------------------------------ WARNING: Ugly macros incoming ------------------------------

#[doc(hidden)]
#[macro_export]
macro_rules! _color_output {
    ($tag:expr, $color:expr, $text:expr, $stderr:expr) => {
        match $stderr {
            true => eprintln!(
                "{}{}[{}]{} {}{}{}",
                $crate::utils::Ansi::BOLD,
                $color.fg,
                $tag,
                $crate::utils::Ansi::RESET,
                $crate::utils::Ansi::BOLD,
                $text,
                $crate::utils::Ansi::RESET
            ),
            false => println!(
                "{}{}[{}]{} {}{}{}",
                $crate::utils::Ansi::BOLD,
                $color.fg,
                $tag,
                $crate::utils::Ansi::RESET,
                $crate::utils::Ansi::BOLD,
                $text,
                $crate::utils::Ansi::RESET
            ),
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        _color_output!("INFORMATION", $crate::utils::Ansi::GREEN, format_args!($($arg)*), false);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        _color_output!("WARNING", $crate::utils::Ansi::YELLOW, format_args!($($arg)*), false);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        _color_output!("ERROR", $crate::utils::Ansi::RED, format_args!($($arg)*), true)

        std::process::exit(1);
    };
}