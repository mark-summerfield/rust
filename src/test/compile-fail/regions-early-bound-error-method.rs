// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Tests that you can use a fn lifetime parameter as part of
// the value for a type parameter in a bound.

trait GetRef<'a> {
    fn get(&self) -> &'a isize;
}

struct Box<'a> {
    t: &'a isize
}

impl<'a> GetRef<'a> for Box<'a> {
    fn get(&self) -> &'a isize {
        self.t
    }
}

impl<'a> Box<'a> {
    fn or<'b,G:GetRef<'b>>(&self, g2: G) -> &'a isize {
        g2.get()
        //~^ ERROR mismatched types
        //~| expected type `&'a isize`
        //~| found type `&'b isize`
        //~| lifetime mismatch

    }
}

fn main() {
}
