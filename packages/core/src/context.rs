use crate::bot::Bot;
use std::sync::Arc;

// TODO: 完善上下文系统
#[derive(Debug, Clone, Default)]
pub struct Context {
    pub bots: Vec<Arc<Bot>>,
}

impl Context {
    pub fn new() -> Self {
        Self { bots: Vec::new() }
    }
}
