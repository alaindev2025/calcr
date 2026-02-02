use clap::builder::styling::{AnsiColor, Effects, Styles};

pub fn get_style() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
        .placeholder(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
}
