// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(associated_types)]

use std::num::Zero;
use std::ops::Add;

#[derive(Zero)]
struct Vector2<T>(T, T);

impl<T: Add<Output=T>> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T> {
        match (self, other) {
            (Vector2(x0, y0), Vector2(x1, y1)) => {
                Vector2(x0 + x1, y0 + y1)
            }
        }
    }
}

#[derive(Zero)]
struct Vector3<T> {
    x: T, y: T, z: T,
}

impl<T: Add<Output=T>> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Zero)]
struct Matrix3x2<T> {
    x: Vector2<T>,
    y: Vector2<T>,
    z: Vector2<T>,
}

impl<T: Add<Output=T>> Add for Matrix3x2<T> {
    type Output = Matrix3x2<T>;

    fn add(self, other: Matrix3x2<T>) -> Matrix3x2<T> {
        Matrix3x2 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

pub fn main() {
    let _: Vector2<int> = Zero::zero();
    let _: Vector3<f64> = Zero::zero();
    let _: Matrix3x2<u8> = Zero::zero();
}
