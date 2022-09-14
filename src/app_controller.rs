use std::time::Duration;

use druid::{TimerToken, Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, Size};

use crate::{app::App, completion_parser::CompletionParser};



pub struct AppController {
    timer_token: Option<TimerToken>,
}

impl AppController {
    pub fn new() -> Self {
        Self {
            timer_token: None,
        }
    }
}

impl Widget<App> for AppController {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, app: &mut App, env: &Env) {
        if let Event::WindowConnected = event {
            self.timer_token = Some(ctx.request_timer(Duration::from_millis(100)));
        }
        else if let Event::Timer(timer_token) = event {
            if self.timer_token.as_ref() == Some(timer_token) {
                if let Some((parser, completion)) = app.brain_receiver().write().unwrap().get_completion(&app.runtime_handle()) {
                    let completion_text = parser.parse(&completion);
                    println!("Found completion from brain receiver {:?}", completion_text);
                    app.log.write_story(completion_text);
                }
                self.timer_token = Some(ctx.request_timer(Duration::from_millis(100)));
            }
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &App, _env: &Env) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &App, _data: &App, _env: &Env) {
    }

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, _data: &App, _env: &Env) -> Size {
        Size::ZERO
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &App, _env: &Env) {
    }
}