# Rust runner
Just some random experiment of running a set of async rust code via a yaml file.

Run the example via `cargo run example`.

See `data/example.yaml` for the example command file and the defined actions in `crates/runner/src/actions`.

## Features
- Support references to other commands as arguments
- Validate arguments
- Validate reference arguments (names and types)
- Support optional arguments

## Todo
- Add 'From' support for return values into argument types
- Add paths in references to return types that are objects
- Support concurrency
- Create planner for orchestrating commands to run based on dependencies and priorities
