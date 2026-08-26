//@aux-build:proc_macros.rs
//@aux-build:non-exhaustive-struct.rs
#![warn(clippy::unnecessary_rest_pattern)]
#![expect(clippy::struct_field_names)]
#![allow(clippy::unneeded_wildcard_pattern)]

use non_exhaustive_struct::{NonExhaustiveStruct, NonExhaustiveStructNoPrivateFields};

struct S {
    a: u8,
    b: u8,
    c: u8,
}

#[derive(Default)]
#[non_exhaustive]
struct LocalNonExhaustive {
    field: i32,
}

enum E {
    A { a1: u8, a2: u8 },
    B { b1: u8, b2: u8 },
    C {},
}

mod m {
    #[derive(Default)]
    pub struct Sm {
        pub a: u8,
        pub(crate) b: u8,
        c: u8,
    }
}

type Variant = VariantKind;

struct VariantKind;

struct A {
    a: i32,
    b: i64,
    c: &'static str,
}

macro_rules! foo {
    ($param:expr) => {
        match $param {
            A { a: 0, b: 0, c: "", .. } => {},
            _ => {},
        }
    };
}

fn main() {
    let a_struct = A { a: 5, b: 42, c: "A" };

    match a_struct {
        A { a: 5, b: 42, c: "", .. } => {}, // Lint
        //~^ unnecessary_rest_pattern
        A { a: 0, b: 0, c: "", .. } => {}, // Lint
        //~^ unnecessary_rest_pattern
        _ => {},
    }

    match a_struct {
        A { a: 5, b: 42, .. } => {},
        A { a: 0, b: 0, c: "", .. } => {}, // Lint
        //~^ unnecessary_rest_pattern
        _ => {},
    }

    // No lint
    match a_struct {
        A { a: 5, .. } => {},
        A { a: 0, b: 0, .. } => {},
        _ => {},
    }

    // No lint
    foo!(a_struct);

    #[non_exhaustive]
    struct B {
        a: u32,
        b: u32,
        c: u64,
    }

    let b_struct = B { a: 5, b: 42, c: 342 };

    match b_struct {
        B { a: 5, b: 42, .. } => {},
        B { a: 0, b: 0, c: 128, .. } => {}, // No Lint
        _ => {},
    }

    let s = S { a: 1, b: 2, c: 3 };

    let S { a, b, c, .. } = s;
    //~^ unnecessary_rest_pattern

    let e = E::A { a1: 1, a2: 2 };

    match e {
        E::A { a1, a2 } => (),
        E::B { b1, b2, .. } => (),
        //~^ unnecessary_rest_pattern
        E::C { .. } => (),
        //~^ unnecessary_rest_pattern
    }

    proc_macros::external! {
        let s1 = S { a: 1, b: 2, c: 3 };
        let S { a, b, c, .. } = s1;
    }

    proc_macros::with_span! {
        span
        let s2 = S { a: 1, b: 2, c: 3 };
        let S { a, b, c, .. } = s2;
    }

    let ne = NonExhaustiveStruct::default();
    let NonExhaustiveStruct { field1: _, .. } = ne;

    let ne = NonExhaustiveStruct::default();
    let NonExhaustiveStruct {
        field1: _, field2: _, ..
    } = ne;

    let ne = NonExhaustiveStructNoPrivateFields::default();
    let NonExhaustiveStructNoPrivateFields { .. } = ne;

    let ne = NonExhaustiveStructNoPrivateFields::default();
    let NonExhaustiveStructNoPrivateFields { field: _, .. } = ne;

    let ne = LocalNonExhaustive::default();
    let LocalNonExhaustive { field: _ } = ne;

    let ne = LocalNonExhaustive::default();
    let LocalNonExhaustive { field: _, .. } = ne;
    //~^ unnecessary_rest_pattern

    let ne = LocalNonExhaustive::default();
    let LocalNonExhaustive { .. } = ne;

    use m::Sm;

    let Sm { .. } = Sm::default();
    let Sm { a: _, b: _, .. } = Sm::default();

    let variant = Variant {};

    let Variant { .. } = variant;
    //~^ unnecessary_rest_pattern

    match variant {
        Variant { .. } => {},
        //~^ unnecessary_rest_pattern
    }
}
