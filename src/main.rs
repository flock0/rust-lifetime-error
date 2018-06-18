extern crate tokio_core;
extern crate futures;

use futures::future::{ok, Future};
use std::boxed::Box;

pub struct SomeStruct{
	some_val: u32
}
impl SomeStruct {
	pub fn do_something(&self, value: u32) -> u32 {
		// Do some work
		return self.some_val + value;
	}
}

fn main() {
	let core = tokio_core::reactor::Core::new().unwrap();
	let my_struct = SomeStruct{some_val:10};

	let future = get_future(&my_struct);

	core.run(future);

	let future2 = get_future(&my_struct);

	core.run(future2);
}

fn get_future(some_struct: & SomeStruct) -> Box<Future<Item=u32, Error=()>> {
	let fut = ok(20).and_then(|val| {
		let result = some_struct.do_something(val);
		ok(result)
	});
	Box::new(fut)
}