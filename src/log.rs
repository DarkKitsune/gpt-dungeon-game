use std::{fmt::Display, rc::Rc, cell::RefCell, sync::{Arc, RwLock}, time::Duration};

use druid::{Data, Color, piet::TextStorage, Event, Widget, widget::Controller, EventCtx, Env, LifeCycleCtx, UpdateCtx, LifeCycle, TimerToken, Size};

use crate::{brain::BrainReceiver, app::App};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LogType {
    Info,
    Story,
    Input,
    Error,
}

impl LogType {
    fn color(&self) -> Color {
        match self {
            LogType::Info => Color::GREEN,
            LogType::Story => Color::WHITE,
            LogType::Input => Color::from_rgba32_u32(0x9FAFFFFF),
            LogType::Error => Color::RED,
        }
    }
}

#[derive(PartialEq, Eq)]
struct LogLine {
    message_type: LogType,
    message: String,
}

impl Display for LogLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

struct LogBase {
    lines: Vec<LogLine>,
}

#[derive(Clone, Data)]
pub struct Log {
    #[data(ignore)]
    base: Arc<RwLock<LogBase>>,
    version: u64,
}

impl Log {
    pub fn new() -> Self {
        Self {
            base: Arc::new(RwLock::new(LogBase {
                lines: Vec::new(),
            })),
            version: 0,
        }
    }
    
    pub fn write(&mut self, message_type: LogType, message: impl Into<String>) {
        self.base.write().unwrap().lines.push(LogLine {
            message_type,
            message: message.into(),
        });
        self.version += 1;
    }

    pub fn write_info(&mut self, message: impl Into<String>) {
        self.write(LogType::Info, message);
    }

    pub fn write_story(&mut self, message: impl Into<String>) {
        self.write(LogType::Story, message);
    }

    pub fn write_input(&mut self, message: impl Into<String>) {
        self.write(LogType::Input, message);
    }

    pub fn write_error(&mut self, message: impl Into<String>) {
        self.write(LogType::Error, message);
    }

    pub fn clear(&mut self) {
        self.base.write().unwrap().lines.clear();
        self.version += 1;
    }

    pub fn using_lines<R, F: FnMut(LogType, &str)>(&self, mut f: F) {
        for line in &self.base.read().unwrap().lines {
            f(line.message_type, &line.message);
        }
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for LogLine { message_type, message } in self.base.read().unwrap().lines.iter() {
            let prefix = match message_type {
                LogType::Info => "INFO: ",
                LogType::Story => "",
                LogType::Input => "> ",
                LogType::Error => "ERROR: ",
            };
            writeln!(f, "{}{}", prefix, message)?;
        }
        Ok(())
    }
}

impl PartialEq for Log {
    fn eq(&self, other: &Self) -> bool {
        self.base.read().unwrap().lines == other.base.read().unwrap().lines
    }
}