use crate::{
    run::RunContext,
    validate::{ValidateContext, ValidateError, Validator},
};
use serde::Deserialize;
use std::{
    any::{type_name, Any},
    marker::PhantomData,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaggedArg<T> {
    Ref(String),
    _Phantom(PhantomData<T>),
}

impl<T> TaggedArg<T> {
    fn value<'a>(&'a self, ctx: &'a RunContext) -> &'a T
    where
        T: Any,
    {
        match self {
            Self::Ref(val) => ctx
                .outputs
                .get(val.as_str())
                .unwrap()
                .downcast_ref::<T>()
                .unwrap(),
            _ => unimplemented!(),
        }
    }
}

impl<T> Validator for TaggedArg<T> {
    fn validate(&self, ctx: &ValidateContext) -> Result<(), ValidateError> {
        #[allow(clippy::single_match)]
        match self {
            Self::Ref(ref_str) => {
                // match the name against the list of commands in the program
                let meta = ctx
                    .commands_meta
                    .get(ref_str)
                    .ok_or_else(|| format!("Ref '{ref_str}' cannot be found"))?;

                // match the type against the return type of the val
                let expected_type = type_name::<T>();
                if meta.output_type != expected_type {
                    return Err(format!(
                        "Ref type mismatch, expected {expected_type}, got {}",
                        meta.output_type
                    )
                    .into());
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Arg<T> {
    TaggedArg(TaggedArg<T>),
    Const(T),
}

impl<T> Arg<T> {
    pub fn value<'a>(&'a self, ctx: &'a RunContext) -> &'a T
    where
        T: Any,
    {
        match self {
            Self::TaggedArg(tagged_arg) => tagged_arg.value(ctx),
            Self::Const(val) => val,
        }
    }
}

impl<T> Validator for Arg<T> {
    fn validate(&self, ctx: &ValidateContext) -> Result<(), ValidateError> {
        #[allow(clippy::single_match)]
        match self {
            Self::TaggedArg(tagged_arg) => {
                tagged_arg.validate(ctx)?;
            }
            _ => {}
        }
        Ok(())
    }
}
