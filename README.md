# never type [![Build Status](https://travis-ci.com/CodeChain-io/rust-never-type.svg?branch=master)](https://travis-ci.com/CodeChain-io/rust-never-type) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
Rust has a type called [never](https://doc.rust-lang.org/std/primitive.never.html)(`!`).
It's used to represent types that are never initiated.

But currently, the never type is a night-only feature and cannot be used in stable builds.
This library is an alternative implementation of the never type for stable Rust.
