use crate::command::CommandMeta;
use std::collections::HashMap;

pub struct ValidateContext<'a> {
    pub commands_meta: HashMap<&'a String, CommandMeta>,
}

impl<'a> ValidateContext<'a> {
    pub fn new(commands_meta: HashMap<&'a String, CommandMeta>) -> Self {
        Self { commands_meta }
    }
}

// just any error for now, should add to the validator trait
pub type ValidateError = Box<dyn std::error::Error>;

pub trait Validator {
    fn validate(&self, ctx: &ValidateContext) -> Result<(), ValidateError>;
}

impl<T: Validator> Validator for Option<T> {
    fn validate(&self, ctx: &ValidateContext) -> Result<(), ValidateError> {
        let Some(val) = self else {
            return Ok(());
        };

        val.validate(ctx)
    }
}
