mod do_something_else;
mod do_that;
mod do_this;

use self::{do_something_else::DoSomethingElse, do_that::DoThat, do_this::DoThis};
use crate::{
    run::{BoxedAny, RunContext, Runnable},
    validate::{ValidateContext, ValidateError, Validator},
};
use serde::Deserialize;
use std::any::type_name;

macro_rules! actions {
    ($($name:ident => $type:ty,)+) => {
        #[derive(Debug, Clone, Deserialize)]
        #[serde(rename_all = "snake_case")]
        #[allow(clippy::enum_variant_names)]
        pub enum Action {
            $(
               $name($type),
            )*
        }

        impl Validator for Action {
            fn validate(&self, ctx: &ValidateContext) -> Result<(), ValidateError> {
                //for ref type we will check the ref exists and it's expected type matches
                match self {
                    $(
                        Action::$name(action) => action.validate(ctx),
                    )*
                }
            }
        }

        #[async_trait::async_trait]
        impl Runnable for Action {
            //we have to cast to Any here as otherwise we will need type constraints to propagate to Arg
            type Output = BoxedAny;

            #[allow(clippy::unit_arg)]
            async fn run(&self, ctx: &RunContext) -> Self::Output {
                match self {
                    $(
                        Action::$name(val) => Box::new(val.run(ctx).await) as BoxedAny,
                    )*
                }
            }
        }

        impl Action {
            pub fn output_type_name(&self) -> &'static str {
                match self {
                    $(
                        Action::$name(_) => type_name::<<$type as Runnable>::Output>(),
                    )*
                }
            }
        }
    };
}

actions!(
    DoSomethingElse => DoSomethingElse,
    DoThis => DoThis,
    DoThat => DoThat,
);
