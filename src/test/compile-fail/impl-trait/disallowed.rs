// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(conservative_impl_trait)]

type Factory<R> = impl Fn() -> R;
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
//~^^ ERROR type parameter `R` is unused

type GlobalFactory<R> = fn() -> impl FnOnce() -> R;
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
//~^^ ERROR type parameter `R` is unused

trait LazyToString {
    fn lazy_to_string<'a>(&'a self) -> impl Fn() -> String;
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
}

impl LazyToString for String {
    fn lazy_to_string<'a>(&'a self) -> impl Fn() -> String {
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
        || self.clone()
    }
}

#[derive(Copy, Clone)]
struct Lazy<T>(T);

impl std::ops::Add<Lazy<i32>> for Lazy<i32> {
    type Output = impl Fn() -> Lazy<i32>;
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

    fn add(self, other: Lazy<i32>) -> Self::Output {
        move || Lazy(self.0 + other.0)
    }
}

impl<F> std::ops::Add<F>
for impl Fn() -> Lazy<i32>
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
where F: Fn() -> impl FnOnce() -> i32
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
{
    type Output = Self;

    fn add(self, other: F) -> Self::Output {
        move || Lazy(self().0 + other()())
    }
}

fn main() {}
