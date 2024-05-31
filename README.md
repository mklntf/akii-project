# AKII Project

The AKII Semester 2 project.

## Unit Tests & Code Coverage

Run `cargo llvm-cov` to execute unit tests and generate a code coverage report.

To generate a html report run `cargo llvm-cov --html`.

## Static Code Analysis

To analyze the code run `cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::pedantic`

## Vulnerability check

Run the `cargo audit` command to check code for vulnerabilities.
