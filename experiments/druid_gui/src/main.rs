use druid::widget::{Align, Button, Flex, DynLabel, Padding};
use druid::{AppLauncher, Widget, WindowDesc};

fn main() {
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<u32> {
    let label = DynLabel::new(|data: &u32, _env| format!("Hello, World for the {} time!", data));
    let button = Button::new("Increment", |_ctx, data, _env| *data += 1);

    Flex::column()
        .with_child(Align::centered(Padding::new(5.0, label)), 1.0)
        .with_child(Padding::new(5.0, button), 1.0)
}
