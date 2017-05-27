#!/bin/sh
set -o errexit -o nounset

# API docs
cargo doc --no-deps
rm -r docs/api
mv -T target/doc docs/api

# Book
mdbook build -d ../docs/book book
