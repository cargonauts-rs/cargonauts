#!/bin/sh
set -o errexit -o nounset

# Book
mdbook build -d ../docs/ book

# API docs
cargo doc --no-deps
mv -T target/doc docs/api
