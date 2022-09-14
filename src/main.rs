use app::App;
use druid::{WindowDesc, AppLauncher};
use tokio::runtime::Handle;

pub mod app;
pub mod ui;
pub mod log;
pub mod brain;
pub mod app_controller;
pub mod completion_parser;
pub mod prompt_builder;

#[tokio::main]
async fn main() {
    let main_window = WindowDesc::new(ui::ui_builder);
    let app = App::new(Handle::current());
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(app)
        .unwrap();
}
