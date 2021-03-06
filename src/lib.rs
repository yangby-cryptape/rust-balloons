// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A rust implementation of [Balloon Hashing].
//!
//! [Balloon Hashing]: https://crypto.stanford.edu/balloon/

mod balloon;
mod error;
mod prelude;

pub use crate::error::{Error, Result};
pub use crate::prelude::{FixedHash, FixedUint, HashAlgo};
pub use balloon::{Balloon, BalloonBuilder};
