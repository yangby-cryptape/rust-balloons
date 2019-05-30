// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{convert::AsRef, default::Default, marker::PhantomData};

use property::Property;

use crate::{FixedHash, FixedUint, HashAlgo, Result};

/// An abstract implementation of [Balloon Hashing].
///
/// Can be created by the `BalloonBuilder`.
pub struct Balloon<U, H, A>
where
    U: FixedUint,
    H: FixedHash<U>,
    A: HashAlgo<U, H>,
{
    /// Space cost (main buffer size)
    space_cost: u64,
    /// Time cost (number of rounds)
    time_cost: u64,
    /// Number of dependencies per block
    delta: u64,

    _u: PhantomData<U>,
    _h: PhantomData<H>,
    _a: PhantomData<A>,
}

/// The builder of `Balloon`.
#[derive(Property)]
#[property(get(disable), set(public, prefix = "", suffix = ""), mut(disable))]
pub struct BalloonBuilder {
    space_cost: u64,
    time_cost: u64,
    delta: u64,
}

impl Default for BalloonBuilder {
    fn default() -> Self {
        Self {
            space_cost: 16,
            time_cost: 20,
            delta: 4,
        }
    }
}

impl BalloonBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build<U, H, A>(&self) -> Balloon<U, H, A>
    where
        U: FixedUint,
        H: FixedHash<U>,
        A: HashAlgo<U, H>,
    {
        let Self {
            space_cost,
            time_cost,
            delta,
        } = *self;
        Balloon {
            space_cost,
            time_cost,
            delta,
            _u: PhantomData,
            _h: PhantomData,
            _a: PhantomData,
        }
    }
}

impl<U, H, A> Balloon<U, H, A>
where
    U: FixedUint,
    H: FixedHash<U>,
    A: HashAlgo<U, H>,
{
    pub fn hash<T1, T2>(&self, passwd: T1, salt: T2) -> Result<H>
    where
        T1: AsRef<[u8]>,
        T2: AsRef<[u8]>,
    {
        let len = self.space_cost as usize;
        let mut cnt = self.space_cost;

        // Create an uninitialized buffer
        let mut buffer = Vec::<H>::with_capacity(len);
        unsafe { buffer.set_len(len) };

        // Step 1. Expand input into buffer
        {
            let mut algo = A::create();
            algo.update_u64(0)?;
            algo.update(&passwd)?;
            algo.update(&salt)?;
            algo.finalize_into(&mut buffer[0])?;

            for idx in 1..len {
                let mut algo = A::create();
                algo.update_u64(idx as u64)?;
                algo.update(&buffer[idx - 1])?;
                algo.finalize_into(&mut buffer[idx])?;
            }
        }

        // Step 2. Mix buffer contents
        for t in 0..self.time_cost {
            for m in 0..len {
                // Step 2a. Hash last and current blocks
                let idx = if m == 0 { len - 1 } else { m - 1 };
                {
                    let mut algo = A::create();
                    algo.update_u64(cnt)?;
                    cnt += 1;
                    algo.update(&buffer[idx])?;
                    algo.update(&buffer[m])?;
                    algo.finalize_into(&mut buffer[m])?;
                }

                // Step 2b. Hash in pseudorandomly chosen blocks
                for i in 0..self.delta {
                    let ints_block = {
                        let mut algo = A::create();
                        algo.update_u64(t)?;
                        algo.update_u64(m as u64)?;
                        algo.update_u64(i)?;
                        algo.finalize()?
                    };
                    let hash_idx = {
                        let mut algo = A::create();
                        algo.update_u64(cnt)?;
                        cnt += 1;
                        algo.update(&salt)?;
                        algo.update(&ints_block)?;
                        algo.finalize()?
                    };
                    let idx = hash_idx.to_int().modulo(self.space_cost) as usize;
                    {
                        let mut algo = A::create();
                        algo.update_u64(cnt)?;
                        cnt += 1;
                        algo.update(&buffer[m])?;
                        algo.update(&buffer[idx])?;
                        algo.finalize_into(&mut buffer[m])?;
                    }
                }
            }
        }

        // Step 3. Extract output from buffer
        Ok(buffer.remove(len - 1))
    }
}
