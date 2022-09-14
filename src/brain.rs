use std::{future::Future, rc::Rc, cell::RefCell, sync::{Arc, RwLock}, pin::Pin, convert::identity};

use druid::{Data};
use openai::api::{CompletionArgs, Completion};
use openai_api_fork as openai;
use tokio::{runtime::Handle, task::{JoinHandle, block_in_place}};

use crate::{log::Log, prompt_builder::{PromptBuilder, Prompt}, completion_parser::{CompletionParser, DynCompletionParser}};

struct BrainBase {
    runtime_handle: Handle,
    openai_client: openai::Client,
    receiver: Arc<RwLock<BrainReceiver>>,
    prompt_builder: PromptBuilder,
}

#[derive(Clone)]
pub struct Brain {
    base: Rc<RefCell<BrainBase>>,
}

impl Brain {
    pub fn new(runtime_handle: Handle) -> Self {
        Self {
            base: Rc::new(RefCell::new(
                BrainBase {
                    runtime_handle,
                    openai_client: openai::Client::new(&std::env::var("OPENAI_SK")
                        .expect("OPENAI_SK not set; no secret key to work with")),
                    receiver: Arc::new(RwLock::new(BrainReceiver::new())),
                    prompt_builder: PromptBuilder::new(),
                }
            ))
        }
    }

    pub fn submit_prompt<P: Prompt + 'static>(&mut self, runtime_handle: &Handle, log: &mut Log, prompt: &P) -> bool {
        // Cancel if already waiting for a completion
        if self.base.borrow().receiver.read().unwrap().waiting() {
            return false;
        }
        // Build prompt
        let args = match self.base.borrow().prompt_builder.build_prompt(log, prompt) {
            Some(args) => args,
            None => return false,
        };
        // Send prompt to OpenAI
        let base = self.base.borrow_mut();
        let client_clone = base.openai_client.clone();
        base.receiver.write().unwrap().completion = Some(
            runtime_handle.spawn(async move {
                (
                    identity::<Box<dyn DynCompletionParser>>(Box::new(CompletionParser::<P>::new())),
                    client_clone
                        .complete_prompt(args)
                        .await,
                )
            })
        );
        true
    }

    pub fn brain_receiver(&self) -> Arc<RwLock<BrainReceiver>> {
        self.base.borrow().receiver.clone()
    }
}

pub struct BrainReceiver {
    completion: Option<JoinHandle<(Box<dyn DynCompletionParser>, Result<Completion, openai::Error>)>>,
}

impl BrainReceiver {
    pub fn new() -> Self {
        Self {
            completion: None,
        }
    }

    pub fn waiting(&self) -> bool {
        if let Some(completion) = &self.completion {
            if completion.is_finished() {
                false
            }
            else {
                true
            }
        }
        else {
            false
        }
    }

    pub fn get_completion(&mut self, runtime_handle: &Handle) -> Option<(Box<dyn DynCompletionParser>, Completion)> {
        if self.waiting() {
            return None;
        }
        self.completion
            .take()
            .map(|c| {
                let (parser, result) = block_in_place(|| runtime_handle.block_on(c))
                    .unwrap();
                (
                    parser,
                    result.unwrap(),
                )
            })
    }
}