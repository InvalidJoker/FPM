pub struct AnsiColor {
    pub fg: &'static str,
    pub bg: &'static str,
}

pub struct Ansi {}
impl Ansi {
    pub const RESET: &'static str = "\x1b[0m";

    pub const BLACK: AnsiColor = AnsiColor { fg: "\x1b[30m", bg: "\x1b[40m" };
    pub const RED: AnsiColor = AnsiColor { fg: "\x1b[31m", bg: "\x1b[41m" };
    pub const GREEN: AnsiColor = AnsiColor { fg: "\x1b[32m", bg: "\x1b[42m" };
    pub const YELLOW: AnsiColor = AnsiColor { fg: "\x1b[33m", bg: "\x1b[43m" };
    pub const BLUE: AnsiColor = AnsiColor { fg: "\x1b[34m", bg: "\x1b[44m" };
    pub const MAGENTA: AnsiColor = AnsiColor { fg: "\x1b[35m", bg: "\x1b[45m" };
    pub const CYAN: AnsiColor = AnsiColor { fg: "\x1b[36m", bg: "\x1b[46m" };
    pub const WHITE: AnsiColor = AnsiColor { fg: "\x1b[37m", bg: "\x1b[47m" };

    pub const BRIGHT_BLACK: AnsiColor = AnsiColor { fg: "\x1b[90m", bg: "\x1b[100m" };
    pub const BRIGHT_RED: AnsiColor = AnsiColor { fg: "\x1b[91m", bg: "\x1b[101m" };
    pub const BRIGHT_GREEN: AnsiColor = AnsiColor { fg: "\x1b[92m", bg: "\x1b[102m" };
    pub const BRIGHT_YELLOW: AnsiColor = AnsiColor { fg: "\x1b[93m", bg: "\x1b[103m" };
    pub const BRIGHT_BLUE: AnsiColor = AnsiColor { fg: "\x1b[94m", bg: "\x1b[104m" };
    pub const BRIGHT_MAGENTA: AnsiColor = AnsiColor { fg: "\x1b[95m", bg: "\x1b[105m" };
    pub const BRIGHT_CYAN: AnsiColor = AnsiColor { fg: "\x1b[96m", bg: "\x1b[106m" };
    pub const BRIGHT_WHITE: AnsiColor = AnsiColor { fg: "\x1b[97m", bg: "\x1b[107m" };

    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
    pub const ITALIC: &'static str = "\x1b[3m";
    pub const UNDERLINE: &'static str = "\x1b[4m";
    pub const BLINK: &'static str = "\x1b[5m";
    pub const REVERSE: &'static str = "\x1b[7m";
    pub const HIDDEN: &'static str = "\x1b[8m";
    pub const STRIKETHROUGH: &'static str = "\x1b[9m";
}

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
    };
}

#[macro_export]
macro_rules! throw {
    ($($arg:tt)*) => {
        error!("{}", format_args!($($arg)*));
        
        std::process::exit(1);
    };
}