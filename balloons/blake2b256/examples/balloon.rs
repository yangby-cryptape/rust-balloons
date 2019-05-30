// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use blake2b256_balloon::Blake2b256BalloonBuilder;

fn main() {
    let mut builder = Blake2b256BalloonBuilder::new();
    builder.space_cost(32).time_cost(40).delta(8);
    let balloon = builder.build();
    let passwd = b"passwd";
    let salt = "salt";
    let mut hash = balloon.hash(passwd, salt).unwrap();
    println!("hash = {:02x?}", hash);
    for _ in 0..64 {
        hash = balloon.hash(&hash, salt).unwrap();
        println!("hash = {:02x?}", hash);
    }
}
