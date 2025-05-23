#![allow(unused_mut, unused, clippy::safe_code)]
#![warn(clippy::missing_mut)]

struct Foo {
    a: (),
}

struct Bar((), (), ());

fn foo(
    a: (),
    //~^ missing_mut
    mut b: (),
    c: (),
    //~^ missing_mut
    ref d: (),
    //~^ missing_mut
    Foo { a: mut e }: Foo,
    Foo { a: f }: Foo,
    //~^ missing_mut
    (
        g,
        //~^ missing_mut
        mut h,
        i,
        //~^ missing_mut
    ): ((), (), ()),
    Bar(
        j,
        //~^ missing_mut
        mut k,
        l,
        //~^ missing_mut
    ): Bar,
) {
    let a = 4;
    //~^ missing_mut
    let mut b = 4;
}

use core::ffi::c_int;

// you can't use patterns in extern blocks
unsafe extern "C" {
    pub unsafe fn printf(fmt: *const u8, ...) -> c_int;
}

static FOO: () = ();
//~^ missing_mut
static mut BAR: () = ();

fn main() {
    // test code goes here
}
