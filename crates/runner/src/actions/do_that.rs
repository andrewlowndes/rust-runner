use crate::{
    run::{RunContext, Runnable},
    validate::{ValidateContext, ValidateError, Validator},
};
use rust_runner_macros::Validated;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Validated)]
pub struct DoThat;

#[async_trait::async_trait]
impl Runnable for DoThat {
    type Output = ();

    async fn run(&self, _ctx: &RunContext) -> Self::Output {
        dbg!("do_that");
    }
}
