use druid::{Widget, widget::{Flex, Label, Button, TextBox}, WidgetExt, TextAlignment};

use crate::{app::App, app_controller::AppController};

pub fn ui_builder() -> impl Widget<App> {
    Flex::column()
        .with_child(log())
        .with_child(inputs())
        .with_child(button())
        .with_child(app_controller())
}

fn log() -> impl Widget<App> {
    Label::new(|app: &App, _env: &_| format!("{}", app.log))
}

fn inputs() -> impl Widget<App> {
    Flex::row()
        .with_child(TextBox::new().with_placeholder("Input").with_text_alignment(TextAlignment::Start).lens(App::input).fix_width(400.0))
        .with_child(TextBox::new().with_placeholder("Language").with_text_alignment(TextAlignment::Start).lens(App::language).fix_width(100.0))
    
}

fn button() -> impl Widget<App> {
    Button::<App>::new("Click me!")
        .on_click(|_, app, _| {
            println!("Clicked!");
            app.submit_input();
        })
}

fn app_controller() -> impl Widget<App> {
    AppController::new()
}