#![feature(proc_macro_hygiene)]

use inline_python::pyo3::{self, prelude::*, wrap_pyfunction};
use inline_python::{python, Context};

#[pyfunction]
fn rust_print(x: i32) {
	println!("rust: x = {}", x);
}

fn main() {
	let c = Context::new();

	c.add_wrapped(wrap_pyfunction!(rust_print));

	c.run(python! {
		x = 123
		print("python: x =", x)
		rust_print(x)
	});
}
