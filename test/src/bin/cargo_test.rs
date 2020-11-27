// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A

#[allow(unused_imports)]
use lib::tests::*;

use std::io::{self, Write, Result};
use std::process::Command;

fn main() -> Result<()> {
	let output = Command::new("cargo")
		.arg("test")
		.arg("--")
		.arg("--test-threads=1")
		.output()?;

	if !output.status.success() {
		println!("`cargo test` failed");
		println!("--- captured stdout ---");
		io::stdout().write_all(&output.stdout)?;
		println!("--- captured stderr ---");
		io::stdout().write_all(&output.stderr)?;
	}

	println!("Hello World");

	Ok(())
}
