#!/usr/bin/env -S just --justfile

# Make the default recipe just list possible recipes. Taken from the
# `just` documentation:
#     https://github.com/casey/just?tab=readme-ov-file#listing-available-recipes
# 
default:
    @just --list --unsorted --justfile {{justfile()}}

alias b := build
alias t := test
alias c := clean
alias r := run
alias p := package

build-dir := 'target/'

build:
    cargo build

test:
    cargo test

package *args:
    cargo package {{args}}

clean:
    rm -r {{build-dir}}

run *args:
    cargo run -- {{args}}

