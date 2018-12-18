# rust-never [![Build Status](https://travis-ci.org/CodeChain-io/rust-never.svg?branch=master)](https://travis-ci.org/CodeChain-io/rust-never) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
Rust has a type called [never](https://doc.rust-lang.org/std/primitive.never.html)(`!`).
It's used to represent the type that is never initiated.

But currently, never type is a night-only feature and cannot be used in the stable build.
This library is an alternative implementation of never type for the stable rust.
