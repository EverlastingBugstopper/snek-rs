#[macro_export]
macro_rules! info {
    ($msg:expr $(, $($tokens:tt)* )?) => {{
        let info_prefix = ansi_term::Colour::White.bold().paint("info:");
        eprintln!(concat!("{} ", $msg), &info_prefix $(, $($tokens)*)*);
    }};
}
