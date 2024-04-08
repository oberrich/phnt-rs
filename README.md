Rust bindings to the [System Informer][sysinf]'s [phnt][docs-rs]
========================================

[![phnt-rs GitHub Actions][github.img]][github]
[![phnt-rs on crates.io][crates-io.img]][crates-io]
[![phnt-rs on docs.rs][docs-rs.img]][docs-rs]

This crate provides Rust with access to the **[System Informer][sysinf]**'s (formerly known as **Process Hacker**) [native Windows headers][sysinf.phnt] (shortened to **phnt**) which provide type definitions, constants, macros as well as function prototypes to even undocumented functions and syscalls.

A nightly copy of the phnt is fetched from [phnt-nightly][phnt.nightly].

**crate version:** 0.0.1 aka work-in-progress.

[github]: https://github.com/oberrich/phnt-rs/actions/workflows/rust.yml?query=branch=master
[github.img]: https://github.com/oberrich/phnt-rs/actions/workflows/rust.yml/badge.svg?branch=master
[crates-io]: https://crates.io/crates/phnt
[crates-io.img]: https://img.shields.io/crates/v/phnt.svg
[docs-rs]: https://docs.rs/phnt
[docs-rs.img]: https://docs.rs/phnt/badge.svg

[sysinf]: https://github.com/winsiderss/systeminformer
[sysinf.phnt]: https://github.com/winsiderss/systeminformer/tree/master/phnt
[phnt.nightly]: https://github.com/oberrich/phnt_nightly
