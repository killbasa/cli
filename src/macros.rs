macro_rules! display {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!(
            "{}",
            format!($($arg)*)
                .if_supports_color(owo_colors::Stream::Stdout, |text| text.bold())
        );
    }};
}
