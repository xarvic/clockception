use druid::{Widget, WidgetExt, UnitPoint, WindowDesc, AppLauncher};
use crate::clock::Clock;
use druid::widget::{Flex, Button};
use crate::position_lens::PositionalLens;

pub mod clock;
pub mod position_lens;
fn clock_row(y: u8) -> impl Widget<u8> {
    Flex::row()
        .with_child(Clock::new().lens(PositionalLens::new(0, y)))
        .with_child(Clock::new().lens(PositionalLens::new(1, y)))
        .with_child(Clock::new().lens(PositionalLens::new(2, y)))
        .with_child(Clock::new().lens(PositionalLens::new(3, y)))
        .with_child(Clock::new().lens(PositionalLens::new(4, y)))
}

fn big_clock() -> impl Widget<u8> {
    Flex::column()
        .with_child(clock_row(0))
        .with_child(clock_row(1))
        .with_child(clock_row(2))
        .with_child(clock_row(3))
        .with_child(clock_row(4))
        .with_child(clock_row(5))
}

fn main_ui() -> impl Widget<u8> {
    let controls = Flex::row()
        .with_child(
            Button::new("IDLE").on_click(|_, b: &mut u8, _|*b = 255)
        )
        .with_default_spacer()
        .with_child(
            Button::new("0").on_click(|_, b: &mut u8, _|*b = 0)
        )
        .with_default_spacer()
        .with_child(
            Button::new("1").on_click(|_, b: &mut u8, _|*b = 1)
        )
        .with_default_spacer()
        .with_child(
            Button::new("2").on_click(|_, b: &mut u8, _|*b = 2)
        )
        .with_default_spacer()
        .with_child(
            Button::new("3").on_click(|_, b: &mut u8, _|*b = 3)
        )
        .with_default_spacer()
        .with_child(
            Button::new("4").on_click(|_, b: &mut u8, _|*b = 4)
        )
        .with_default_spacer()
        .with_child(
            Button::new("5").on_click(|_, b: &mut u8, _|*b = 5)
        )
        .with_default_spacer()
        .with_child(
            Button::new("6").on_click(|_, b: &mut u8, _|*b = 6)
        )
        .with_default_spacer()
        .with_child(
            Button::new("7").on_click(|_, b: &mut u8, _|*b = 7)
        )
        .with_default_spacer()
        .with_child(
            Button::new("8").on_click(|_, b: &mut u8, _|*b = 8)
        )
        .with_default_spacer()
        .with_child(
            Button::new("9").on_click(|_, b: &mut u8, _|*b = 9)
        );
    Flex::column()
        .with_flex_child(big_clock().align_horizontal(UnitPoint::CENTER), 1.0)
        .with_default_spacer()
        .with_child(controls)
        .with_default_spacer()
}

fn main() {
    let window = WindowDesc::new(main_ui());

    AppLauncher::with_window(window).launch(
        0
    ).unwrap();
}
