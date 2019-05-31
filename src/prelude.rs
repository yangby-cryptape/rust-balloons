// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::convert::AsRef;

use crate::Result;

/// A hash algorithm which is used in [balloon hashing algorithm].
///
/// [balloon hashing algorithm]: https://crypto.stanford.edu/balloon/
pub trait HashAlgo<U, H>
where
    U: FixedUint,
    H: FixedHash<U>,
{
    fn create() -> Self;
    fn update<T>(&mut self, data: T) -> Result<()>
    where
        T: AsRef<[u8]>;
    fn finalize(self) -> Result<H>;
    fn finalize_into(self, dst: &mut H) -> Result<()>;

    fn update_u64(&mut self, uint64: u64) -> Result<()> {
        self.update(uint64.to_le_bytes())
    }
}

/// The return type of [balloon hashing algorithm].
///
/// Also, it's the return type of the [internal hash algorithm].
///
/// [balloon hashing algorithm]: https://crypto.stanford.edu/balloon/
/// [internal hash algorithm]: trait.HashAlgo.html
pub trait FixedHash<U>: AsRef<[u8]>
where
    U: FixedUint,
{
    /// Convert to the associated unsigned integer.
    fn to_int(&self) -> U {
        U::from_slice(self.as_ref())
    }
}

/// A unsigned integer which can be converted from [the return] of the [internal hash algorithm].
///
/// [the return]: trait.FixedHash.html
/// [internal hash algorithm]: trait.HashAlgo.html
pub trait FixedUint {
    /// Convert from slice.
    fn from_slice(slice: &[u8]) -> Self;
    /// Find the remainder after division of `self` by `divisor`.
    fn modulo(&self, divisor: u64) -> u64;
}
