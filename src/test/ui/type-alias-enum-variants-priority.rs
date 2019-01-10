#![feature(type_alias_enum_variants)]

enum E {
    V(u8)
}

trait Tr {
    type V;
    fn f() -> Self::V;
}

impl Tr for E {
    type V = u8;
    fn f() -> Self::V { loop {} } //~ ERROR type vs variant ambiguity
}

fn main() {}
