use crate::{
    args::Arg,
    run::{RunContext, Runnable},
    validate::{ValidateContext, ValidateError, Validator},
};
use rust_runner_macros::Validated;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Validated)]
pub struct DoSomethingElse {
    name: Arg<String>,
}

#[async_trait::async_trait]
impl Runnable for DoSomethingElse {
    type Output = ();

    async fn run(&self, ctx: &RunContext) -> Self::Output {
        dbg!("do_something_else", self.name.value(ctx));
    }
}
