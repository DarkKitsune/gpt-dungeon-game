use openai_api_fork::api::{CompletionArgs, CompletionArgsBuilder, Completion};

use crate::log::Log;

/// Builds prompts to send to OpenAI.
pub struct PromptBuilder {

}

impl PromptBuilder {
    /// Create a new prompt builder
    pub fn new() -> Self {
        Self {

        }
    }

    /// Builds a prompt to be sent to OpenAI.
    pub fn build_prompt<P: Prompt>(&self, log: &mut Log, prompt: &P) -> Option<CompletionArgs> {
        match P::create_completion_args_builder(prompt).build() {
            Ok(args) => Some(args),
            Err(e) => {
                log.write_error(format!("Failed to build prompt: {}", e));
                None
            },
        }
    }
}

/// A prompt to be sent to OpenAI.
pub trait Prompt: Send {
    /// Create a `CompletionArgsBuilder` from the prompt.
    fn create_completion_args_builder(&self) -> CompletionArgsBuilder;

    /// Parse the completion returned by OpenAI.
    fn parse_completion(completion: &Completion) -> String;
}

/// A text completion prompt.
pub struct CompletionPrompt {
    text: String,
    max_tokens: u64,
    randomness: f64,
    stop: Vec<String>,
}

impl CompletionPrompt {
    /// Create a new text completion prompt.
    pub fn new(text: impl Into<String>, max_tokens: u64, randomness: f64, stop: Vec<impl Into<String>>) -> Self {
        Self {
            text: text.into(),
            max_tokens,
            randomness,
            stop: stop.into_iter().map(|s| s.into()).collect(),
        }
    }
}

impl Prompt for CompletionPrompt {
    fn create_completion_args_builder(&self) -> CompletionArgsBuilder {
        CompletionArgs::builder()
            .engine("text-davinci-002")
            .prompt(self.text.clone())
            .max_tokens(self.max_tokens)
            .temperature(self.randomness)
            .stop(self.stop.clone())
    }

    fn parse_completion(completion: &Completion) -> String {
        // Just use the first choice
        completion.choices[0].text.clone()
    }
}

/// A translation prompt.
pub struct TranslationPrompt {
    text: String,
    language: String,
    max_tokens: u64,
}

impl TranslationPrompt {
    /// Create a new translation prompt.
    pub fn new(text: impl Into<String>, language: impl Into<String>) -> Self {
        let text = text.into();
        // Estimate the number of tokens needed (Very rough estimate but better than a fixed number)
        let max_tokens = (text.chars().count() as u64 / 2).min(2048);
        Self {
            text,
            language: language.into(),
            max_tokens,
        }
    }
}

impl Prompt for TranslationPrompt {
    fn create_completion_args_builder(&self) -> CompletionArgsBuilder {
        CompletionArgs::builder()
            .engine("text-davinci-002")
            .prompt(format!("Please translate the following text into {}: {} Translation:", self.language, self.text))
            .max_tokens(self.max_tokens)
            .temperature(0.4)
    }

    fn parse_completion(completion: &Completion) -> String {
        // Just use the first choice
        completion.choices[0].text.clone()
    }
}