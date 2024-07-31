/// Terminal User Interface module.
///
/// This file contains public helpers to display formatted information on the terminal.

/// Prints the software opening.
#[macro_export]
macro_rules! tui_opening {
    () => {

        let ascii_art = format!("\t\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
                            "██╗  ██╗ ███████╗ ██████╗  ███╗   ███╗ ███████╗ ███████╗",
                            "██║  ██║ ██╔════╝ ██╔══██╗ ████╗ ████║ ██╔════╝ ██╔════╝",
                            "███████║ █████╗   ██████╔╝ ██╔████╔██║ █████╗   ███████╗",
                            "██╔══██║ ██╔══╝   ██╔══██╗ ██║╚██╔╝██║ ██╔══╝   ╚════██║",
                            "██║  ██║ ███████╗ ██║  ██║ ██║ ╚═╝ ██║ ███████╗ ███████║",
                            "╚═╝  ╚═╝ ╚══════╝ ╚═╝  ╚═╝ ╚═╝     ╚═╝ ╚══════╝ ╚══════╝",
                            "           A (simple) Robot Management System           "
        );
        println!("{}", ascii_art.gradient(colorful::Color::Red));
        println!("\t{}",    "        https://github.com/dclause/hermes-studio        ".white().italic().dim());
        println!("\t{}\n",  "========================================================".gradient(colorful::Color::Yellow).bold());
    };
}

#[macro_export]
macro_rules! tui_trace {
    ($sentence:expr) => {
        println!(
            "{} {}",
            String::from("[>]")
                .color(colorful::Color::MediumPurple)
                .bold(),
            $sentence
        );
        log::trace!("{}", $sentence);
    };
    ($sentence:expr, $data:expr) => {
        println!(
            "{} {}: {}",
            String::from("[>]")
                .color(colorful::Color::MediumPurple)
                .bold(),
            $sentence,
            $data.color(colorful::Color::MediumPurple).italic()
        );
        log::trace!("{}: {}", $sentence, $data);
    };
}

#[macro_export]
macro_rules! tui_info {
    ($sentence:expr) => {
        println!(
            "{} {}",
            "[~]".color(colorful::Color::Blue).bold(),
            $sentence
        );
        log::info!("{}", $sentence);
    };
    ($sentence:expr, $data:expr) => {
        println!(
            "{} {}: {}",
            "[~]".color(colorful::Color::Blue).bold(),
            $sentence,
            $data.color(colorful::Color::Blue).italic()
        );
        log::info!("{}: {}", $sentence, $data);
    };
}

#[macro_export]
macro_rules! tui_warn {
    ($sentence:expr) => {
        println!(
            "{} {}",
            "[!]".color(colorful::Color::Orange1).bold(),
            $sentence
        );
        log::warn!("{}: {}", $sentence, $data);
    };
    ($sentence:expr, $data:expr) => {
        println!(
            "{} {}: {}",
            "[!]".color(colorful::Color::Orange1).bold(),
            $sentence,
            $data.color(colorful::Color::Orange1).italic()
        );
        log::warn!("{}: {}", $sentence, $data);
    };
}

#[macro_export]
macro_rules! tui_error {
    ($sentence:expr) => {
        println!("{} {}", "[x]".color(colorful::Color::Red).bold(), $sentence);
        log::error!("{}", $sentence);
    };
    ($sentence:expr, $data:expr) => {
        println!(
            "{} {}: {}",
            "[x]".color(colorful::Color::Red).bold(),
            $sentence,
            $data.color(colorful::Color::Red).italic()
        );
        log::error!("{}: {}", $sentence, $data);
    };
}

#[macro_export]
macro_rules! tui_critical {
    ($sentence:expr) => {
        println!(
            "{}",
            format!("{} {}", "[x]", $sentence)
                .color(colorful::Color::Red)
                .bold()
        );
        log::error!("{}", $sentence);
    };
    ($sentence:expr, $data:expr) => {
        println!(
            "{} {}",
            format!("[x] {}:", $sentence)
                .color(colorful::Color::Red)
                .bold(),
            $data.color(colorful::Color::Red).italic().bold()
        );
        log::error!("{}: {}", $sentence, $data);
    };
}

#[macro_export]
macro_rules! tui_success {
    ($sentence:expr) => {
        println!(
            "{} {}",
            "[✓]".color(colorful::Color::Green).bold(),
            $sentence
        );
        log::info!("{}", $sentence);
    };
    ($sentence:expr, $data:expr) => {
        println!(
            "{} {}: {}",
            "[✓]".color(colorful::Color::Green).bold(),
            $sentence,
            $data.color(colorful::Color::Green).italic()
        );
        log::info!("{}: {}", $sentence, $data);
    };
}

/// Prints a sentence prefixed with a colored symbol.
/// NOTE: this is a generalization of tui_info!, tui_warn!, tui_error!, tui_success! etc...
#[macro_export]
macro_rules! tui_print {
    ($symbol:expr, $sentence:expr, $color:expr) => {
        println!("{} {}", format!("[{}]", $symbol).color($color).bold(), $sentence)
        log::info!("{}", $sentence);
    };
    ($symbol:expr, $sentence:expr, $data:expr, $color:expr) => {
        println!("{} {}: {}", format!("[{}]", $symbol).color($color).bold(), $sentence, $data.color($color).italic())
        log::info!("{}: {}", $sentence, $data);
    };
}

/// Formats a string, just like format! but using colorization according to the log level.
#[macro_export]
macro_rules! tui_color {
    ($level:expr, $data:expr, $($args:tt)*) => {
        match $level {
            log::Level::Trace => format!($data, $($args)*).color(colorful::Color::MediumPurple),
            log::Level::Debug => format!($data, $($args)*).color(colorful::Color::Blue),
            log::Level::Info => format!($data, $($args)*).color(colorful::Color::LightGray),
            log::Level::Warn => format!($data, $($args)*).color(colorful::Color::Orange1),
            log::Level::Error => format!($data, $($args)*).color(colorful::Color::Red).bold(),
        }
    };
}
