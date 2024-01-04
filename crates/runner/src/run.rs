use std::{any::Any, collections::HashMap};

pub type BoxedAny = Box<dyn Any + Send + Sync>;

#[derive(Default)]
pub struct RunContext<'a> {
    pub outputs: HashMap<&'a str, BoxedAny>,
}

impl<'a> RunContext<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait::async_trait]
pub trait Runnable {
    type Output: Any;

    async fn run(&self, ctx: &RunContext) -> Self::Output;
}
