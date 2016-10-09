use event;
use theme;

// Module is not named `ncurses` to avoir naming conflict
#[cfg(feature = "ncurses")]
pub mod curses;
#[cfg(feature = "termion")]
pub mod termion;

pub trait Backend {
    fn init() -> Self;
    // TODO: take `self` by value?
    fn finish(&mut self);

    fn clear(&self);
    fn refresh(&mut self);

    fn has_colors(&self) -> bool;

    fn init_color_style(&mut self, style: theme::ColorStyle,
                        foreground: &theme::Color, background: &theme::Color);

    fn print_at(&self, (usize, usize), &str);

    fn poll_event(&self) -> event::Event;
    fn set_refresh_rate(&mut self, fps: u32);
    fn screen_size(&self) -> (usize, usize);

    // TODO: unify those into a single method?
    fn with_color<F: FnOnce()>(&self, color: theme::ColorStyle, f: F);
    fn with_effect<F: FnOnce()>(&self, effect: theme::Effect, f: F);
}