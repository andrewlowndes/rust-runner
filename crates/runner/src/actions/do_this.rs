use crate::{
    args::Arg,
    run::{RunContext, Runnable},
    validate::{ValidateContext, ValidateError, Validator},
};
use rust_runner_macros::Validated;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Validated)]
pub struct DoThis {
    name: Arg<String>,
    count: Option<Arg<u32>>,
}

#[async_trait::async_trait]
impl Runnable for DoThis {
    type Output = String;

    async fn run(&self, ctx: &RunContext) -> Self::Output {
        let name = self.name.value(ctx);
        let count = self.count.as_ref().map(|count| *count.value(ctx));

        dbg!("do_this", name, count);
        format!("result_{name}_{}", count.unwrap_or_default())
    }
}
