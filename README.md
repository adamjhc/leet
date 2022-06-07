# leet

A tool to quickly create and open projects for LeetCode solutions

## Motivation

Cargo doesn't allow package names to start with numbers but folder names starting with numbers make for very easy sorting in filesystems so I previously spent a lot of time manually typing out the folder and project name. This project simplifies that into one command. Yes, this probably could have been a bash script.

## Usage

```sh
leet "496. Next Greater Element I"
```

The input gets converted and the following commands are then run:

```sh
cargo new 0496_next-greater-element-i --name next-greater-element-i --lib
code .\0496_next-greater-element-i\
```

## Features

- Blazing fast
- Written in Rust
- Zero dependencies

## Maybe Future features

- [ ] Generate project with a template using [cargo-generate](https://crates.io/crates/cargo-generate)
- [ ] Be code editor agnostic (how much do people use `$EDITOR`?)
