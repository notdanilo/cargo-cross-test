//! Run native and web tests with a single command.
//! Use `cargo cross-test` or `cargo cross-test all` to run tests for all available platforms.
//! Use `cargo cross-test web` to run tests for `wasm-bindgen-tests` tests.
//! Use `cargo cross-test native` to run native tests.

use std::process::Command;
use std::env::Args;

#[derive(PartialEq)]
enum Context {
    All,
    Native,
    Web
}

impl From<Args> for Context {
    fn from(args: Args) -> Self {
        let args: Vec<String> = args.collect();
        if let Some(arg) = args.get(2) {
            let arg = arg.to_lowercase();
            if arg == "native" {
                Context::Native
            } else if arg == "web" {
                Context::Web
            } else {
                Context::All
            }
        } else {
            Context::All
        }
    }
}

fn main() {
    let context: Context = std::env::args().into();

    if context != Context::Web {
        Command::new("cargo")
            .arg("test")
            .status()
            .expect("Failed to execute native test.");
    }

    if context != Context::Native {
        Command::new("rustup")
            .arg("run")
            .arg("nightly")
            .arg("wasm-pack")
            .arg("test")
            .arg("--firefox")
            .arg("--headless")
            .status()
            .expect("Failed to execute web test.");
    }
}

#[cfg(test)]
mod tests {
    use cross_test::prelude::*;

    cross_test_configure!();

    #[cross_test]
    async fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
