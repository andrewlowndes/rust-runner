use crate::{
    actions::Action,
    validate::{ValidateContext, ValidateError, Validator},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    pub name: Option<String>,
    #[serde(flatten)]
    pub action: Action,
}

impl Command {
    // return information about the command such as it's return type and argument types
    pub fn meta(&self) -> CommandMeta {
        CommandMeta {
            output_type: self.action.output_type_name(),
        }
    }
}

impl Validator for Command {
    fn validate(&self, ctx: &ValidateContext) -> Result<(), ValidateError> {
        self.action.validate(ctx)
    }
}

pub struct CommandMeta {
    pub output_type: &'static str,
}
