// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[doc(hidden)]
pub mod kernel;

use balloons::{Balloon, BalloonBuilder};

/// The builder of [`Blake2b256Balloon`].
///
/// [`Blake2b256Balloon`]: type.Blake2b256Balloon.html
#[derive(Default)]
pub struct Blake2b256BalloonBuilder(BalloonBuilder);

/// Balloon Hashing based on blake2b-256.
///
/// Can be created by [`Blake2b256BalloonBuilder`].
///
/// [`Blake2b256BalloonBuilder`]: struct.Blake2b256BalloonBuilder.html
pub type Blake2b256Balloon = Balloon<kernel::U256, kernel::H256, kernel::Blake2b256>;

impl ::std::ops::Deref for Blake2b256BalloonBuilder {
    type Target = BalloonBuilder;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for Blake2b256BalloonBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsMut<BalloonBuilder> for Blake2b256BalloonBuilder {
    fn as_mut(&mut self) -> &mut BalloonBuilder {
        &mut self.0
    }
}

impl Blake2b256BalloonBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(&self) -> Blake2b256Balloon {
        self.0.build()
    }
}
