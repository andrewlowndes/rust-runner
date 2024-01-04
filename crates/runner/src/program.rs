use crate::{
    command::{Command, CommandMeta},
    validate::{ValidateContext, ValidateError, Validator},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Program {
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    pub commands: Vec<Command>,
}

impl Program {
    pub fn validate(&self) -> Result<(), ValidateError> {
        //validate the args before we try before running
        let commands_meta = self
            .commands
            .iter()
            .filter_map(|command| command.name.as_ref().map(|name| (name, command.meta())))
            .collect::<HashMap<&String, CommandMeta>>();

        let ctx = ValidateContext::new(commands_meta);

        for command in &self.commands {
            command.validate(&ctx)?;
        }

        Ok(())
    }
}
