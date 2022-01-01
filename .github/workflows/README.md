# Workflows

## audit

Runs every day `cargo audit`.

## build_and_test

Builds all crates (iota_client and bindings) and runs tests for them when there is a commit to a pull request or push to the dev branch.

## clippy

Runs `cargo clippy check` on all crates (iota_client and bindings) when there is a commit to a pull request or push to the dev branch.

## covector-status

Runs on pull requests and checks if a pull request with a change or a relaese needs to be created.

## covector-version-or-publish

Runs on push to dev and creates a release when necessary.

## deploy-docs-to-gh-pages

Runs on pull request or pushes to the dev or mainnet branch and will deploy the docs from the documentation folder.

## format

Runs `cargo fmt check` on all crates (iota_client and bindings) bindings when there is a commit to a pull request or push to the dev branch.

## python_binding_publish

Builds python wheels on pushes to the dev or mainnet branch.

## udeps

Checks for unused dependencies on pull requests or pushes to the dev or mainnet branch.
