use azul::{prelude::*, widgets::{label::Label, button::Button}};

fn main() {
    let mut app = App::new(HelloWorld { count: 0 }, AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    app.run(window).unwrap();
}

struct HelloWorld {
    count: usize,
}

impl Layout for HelloWorld {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("Hello, World for the {} time!", self.count)).dom();
        let button = Button::with_label("Increment").dom()
            .with_callback(On::MouseUp, increment);

        Dom::div()
            .with_child(label)
            .with_child(button)
    }
}

fn increment(event: CallbackInfo<HelloWorld>) -> UpdateScreen {
    event.state.data.count += 1;
    Redraw
}
