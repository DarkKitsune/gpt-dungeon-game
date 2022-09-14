use std::{future::Future, sync::{RwLock, Arc}, thread, time::Duration};

use druid::{Data, Lens};
use tokio::runtime::{Handle};

use crate::{log::Log, brain::{Brain, BrainReceiver}, prompt_builder::{CompletionPrompt, TranslationPrompt}};

#[derive(Data, Clone, Lens)]
pub struct App {
    #[lens(ignore)]
    #[data(ignore)]
    runtime_handle: Handle,
    #[lens(ignore)]
    #[data(ignore)]
    brain: Brain,
    pub log: Log,
    pub input: String,
    pub language: String,
}

impl App {
    pub fn new(runtime_handle: Handle) -> Self {
        Self {
            runtime_handle: runtime_handle.clone(),
            brain: Brain::new(runtime_handle),
            log: Log::new(),
            input: String::new(),
            language: String::new(),
        }
    }

    pub fn submit_input<'a>(&'a mut self) -> bool {
        let input = self.input.clone();
        self.log.write_input(input.clone());
        if self.brain
            .submit_prompt(
                &self.runtime_handle,
                &mut self.log,
                &TranslationPrompt::new(
                    input,
                    self.language.clone(),
                )
            )
        {
            self.input.clear();
            true
        }
        else {
            false
        }
    }

    pub fn runtime_handle(&self) -> &Handle {
        &self.runtime_handle
    }

    pub fn brain_receiver(&self) -> Arc<RwLock<BrainReceiver>> {
        self.brain.brain_receiver()
    }
}