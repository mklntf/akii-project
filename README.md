# AKII Project

The AKII Semester 2 project.

## Build

Run `cargo build` to build the project. Afterwards run it by executing `./target/debug/akii-project`.
By default the webserver can be reached on `http://localhost:8080` and the database is created in `/tmp/tasks.db`.
The port can be changed via the `SERVER_PORT` environment variable.
The database location can be changed via the `DATABASE_URL` environment variable.

## Unit Tests & Code Coverage

Run `cargo llvm-cov` to execute unit tests and generate a code coverage report.

To generate a html report run `cargo llvm-cov --html`.

## Static Code Analysis

To analyze the code run `cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::pedantic`

## Vulnerability check

Run the `cargo audit` command to check code for vulnerabilities.
