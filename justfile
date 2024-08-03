# Command runner for executing common tasks.
# Similar to a Makefile, but better.
#
# Requires the "just" CLI.
# See https://github.com/casey/just for installation instructions.

set dotenv-load

# Running "just" should list all commands.
# Set to private because the command should not be listed.
[private]
default:
  @just --list

run +ARGS:
  cargo run -p linearly -- {{ARGS}}

# Format the code.
fmt:
  cargo fmt --all

build-debug:
  cargo build

build-release:
  cargo build --release

lint: lint-fmt lint-rust lint-clippy

lint-fmt:
  cargo fmt --version
  cargo fmt --check

lint-rust:
  cargo --version
  cargo check

lint-clippy:
  cargo clippy --version
  cargo clippy -- --deny warnings

# Fix lints (rustfmt + clippy)
fix:
  cargo clippy --fix
  cargo fmt

test:
  cargo test --all

test-ci: test

# Run all lints and tests to ensure CI will pass.
prepush: lint test-ci


# Update the Linear GraphQL schema - download schema from API.
update-api-schema:
  npx get-graphql-schema https://api.linear.app/graphql > ./crates/api/schema.graphql
