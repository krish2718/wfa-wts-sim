# WFA WTS simulator

WTS simulator for unit test CA + DUT by running individual command through an interactive shell.

This is written in Rust and uses cargo as the build system.

## Install
Install Rust using `rustup` from [here](https://www.rust-lang.org/tools/install). This will install `rustc` and `cargo` on your system.


## Build (using cargo)
Use the following command to build the project

```shell
    $ cargo build --release
```

## Usage

```shell
$ ./target/release/wfa-wts-sim -h
IP address and port of CA

Usage: wfa-wts-sim --ca <CA> --port <PORT>

Options:
  -c, --ca <CA>
  -p, --port <PORT>
  -f, --cmd-file <CMD_FILE>
  -h, --help                 Print help
```

## Unit tests and coverage

This application has few unit tests. To run the unit tests, use the following command

```shell
    $ cargo test
```

Code coverage is generated using `cargo-llvm-cov` crate.

After running the unit tests, the code coverage report can be generated using the following command

```shell
    $ cargo test
    $ cargo llvm-cov
    $ cargo llvm-cov --open
```

See [Usage](https://github.com/taiki-e/cargo-llvm-cov#usage) for more details.

## Known issues

None
