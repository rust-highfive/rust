// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This actually tests a lot more than just encodable/decodable, but it gets the
// job done at least

// ignore-test FIXME(#5121)

#![feature(struct_variant)]

extern crate rand;
extern crate rbml;
extern crate serialize;

use std::io::MemWriter;
use rand::{random, Rand};
use rbml;
use rbml::Doc;
use rbml::writer::Encoder;
use rbml::reader::Decoder;
use serialize::{Encodable, Decodable};

#[deriving(Encodable, Decodable, Eq, Rand)]
struct A;
#[deriving(Encodable, Decodable, Eq, Rand)]
struct B(int);
#[deriving(Encodable, Decodable, Eq, Rand)]
struct C(int, int, uint);

#[deriving(Encodable, Decodable, Eq, Rand)]
struct D {
    a: int,
    b: uint,
}

#[deriving(Encodable, Decodable, Eq, Rand)]
enum E {
    E1,
    E2(uint),
    E3(D),
    E4{ x: uint },
}

#[deriving(Encodable, Decodable, Eq, Rand)]
enum F { F1 }

#[deriving(Encodable, Decodable, Eq, Rand)]
struct G<T> {
    t: T
}

fn roundtrip<'a, T: Rand + Eq + Encodable<Encoder<'a>> +
                    Decodable<Decoder<'a>>>() {
    let obj: T = random();
    let mut w = MemWriter::new();
    let mut e = Encoder::new(&mut w);
    obj.encode(&mut e);
    let doc = rbml::Doc::new(@w.get_ref());
    let mut dec = Decoder::new(doc);
    let obj2 = Decodable::decode(&mut dec);
    assert!(obj == obj2);
}

pub fn main() {
    roundtrip::<A>();
    roundtrip::<B>();
    roundtrip::<C>();
    roundtrip::<D>();

    for _ in range(0, 20) {
        roundtrip::<E>();
        roundtrip::<F>();
        roundtrip::<G<int>>();
    }
}
