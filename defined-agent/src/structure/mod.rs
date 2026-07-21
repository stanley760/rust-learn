pub mod act_plan;
mod loop_state;

pub use loop_state::{LoopState, get_llm_client, get_model, extract_text};


pub const MAX_RECOVERY_ATTEMPTS: u32 = 3;
pub const BACKOFF_BASE_DELAY_SECS: f64 = 1.0;
pub const BACKOFF_MAX_DELAY_SECS: f64 = 30.0;
pub const CONTEXT_THRESHOLD_CHARS: usize = 50_000;
pub const CONTINUATION_MESSAGE: &str = "Output limit hit. Continue directly from where you stopped. \
No recap, no repetition. Pick up mid-sentence if needed.";