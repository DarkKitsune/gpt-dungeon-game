use std::marker::PhantomData;

use openai_api_fork as openai;

use openai::api::Completion;

use crate::prompt_builder::Prompt;

pub struct CompletionParser<P: Prompt> {
    _phantom: PhantomData<P>,
}

impl<P: Prompt> CompletionParser<P> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn parse(completion: &Completion) -> String {
        P::parse_completion(&completion)
    }
}

pub trait DynCompletionParser: Send {
    fn parse(&self, completion: &Completion) -> String;
}

impl<P: Prompt> DynCompletionParser for CompletionParser<P> {
    fn parse(&self, completion: &Completion) -> String {
        Self::parse(completion)
    }
}