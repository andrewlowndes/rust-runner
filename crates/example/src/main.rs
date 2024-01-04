use rust_runner::{Program, RunContext, Runnable, ValidateError};

#[tokio::main]
async fn main() -> Result<(), ValidateError> {
    //we can load this from anywhere async, validate it and then run its commands when we like
    let program: Program =
        serde_yaml::from_str(include_str!("../../../data/example.yaml")).unwrap();
    program.validate()?;

    //now we have a program we can run it
    let mut ctx = RunContext::new();

    for command in &program.commands {
        //when we get a response we should persist it
        let result = command.action.run(&ctx).await;

        //add the result for the action to the list of responses if there is a name given
        if let Some(name) = &command.name {
            ctx.outputs.insert(name.as_str(), result);
        }
    }

    Ok(())
}
