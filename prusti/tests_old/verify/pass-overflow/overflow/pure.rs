extern crate prusti_contracts;

#[pure]
#[requires="a + b <= std::u32::MAX"]
fn sum_pure(a: u32, b: u32) -> u32 {
    a + b
}

#[trusted]
#[pure]
fn u32_foo() -> u32 {
    5
}

fn u32_foo_call_1() {
    let n = u32_foo();
    assert!(0 <= n);
    assert!(n <= 4294967295);
}

fn u32_foo_call_2() {
    assert!(0 <= u32_foo());
    assert!(u32_foo() <= 4294967295);
}

#[ensures="0 <= u32_foo() && u32_foo() <= 4294967295"]
fn u32_foo_call_3() {
}

fn main() {}
