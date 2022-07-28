#![feature(backtrace)]
use std::backtrace::Backtrace;

// This test is for backtrace behavior
fn main() {
    let bt = Backtrace::force_capture();
    test1();
    println!("top level backtrace:{:#?}\n\n", bt);
}

fn test1() {
    test2()
}

fn test2() {
    test3()
}

fn test3() {
    test4()
}

fn test4() {
    let bt = Backtrace::force_capture();
    println!("innermost backtrace:{:#?}\n\n", bt);
}
