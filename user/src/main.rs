// if these aren't commented out, it compiles fine
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

use library::*;

fn main() {
    Light.send(MyMessage);
}
