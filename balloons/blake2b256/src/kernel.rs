// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{
    convert::{AsMut, AsRef},
    mem, ptr,
};

use balloons::{FixedHash, FixedUint, HashAlgo, Result};

pub use balloons::BalloonBuilder;

#[derive(Debug)]
pub struct U256([u64; 4]);

#[derive(Debug)]
pub struct H256([u8; 32]);

const BYTE_BITS: usize = 8;

impl U256 {
    #[inline]
    fn bytes_size() -> usize {
        32
    }

    const fn unit_bytes_size() -> usize {
        64 / BYTE_BITS
    }
}

impl FixedUint for U256 {
    fn from_slice(slice: &[u8]) -> Self {
        let bytes_size = Self::bytes_size();
        assert!(slice.len() >= bytes_size);
        let inner = unsafe {
            let mut inner: [u64; 4] = mem::uninitialized();
            let dst_ptr = inner.as_mut_ptr() as *mut u8;
            let src_ptr = slice.as_ptr();
            if cfg!(target_endian = "little") {
                let unit_bytes_size = Self::unit_bytes_size();
                let dst_ptr = dst_ptr.add(bytes_size);
                let src_ptr = src_ptr.offset(-(unit_bytes_size as isize));
                let mut idx = bytes_size;
                loop {
                    let part_dst_ptr = dst_ptr.offset(-(idx as isize));
                    let part_src_ptr = src_ptr.add(idx + unit_bytes_size - 1);
                    let mut part_idx = unit_bytes_size as isize - 1;
                    loop {
                        *part_dst_ptr.offset(part_idx) = *part_src_ptr.offset(-part_idx);
                        if part_idx == 0 {
                            break;
                        } else {
                            part_idx -= 1;
                        }
                    }
                    if idx == unit_bytes_size {
                        break;
                    }
                    idx -= unit_bytes_size;
                }
            } else {
                ptr::copy_nonoverlapping(src_ptr, dst_ptr, bytes_size);
            }
            inner
        };
        U256(inner)
    }

    fn modulo(&self, divisor: u64) -> u64 {
        let m_2_64 = {
            let x = (!0) % divisor + 1;
            if x == divisor {
                0
            } else {
                u128::from(x)
            }
        };
        let tmp = u128::from(self.0[3] % divisor);
        // Suppose:
        //      N = 2^64
        // Since:
        //      tmp <= N-2
        //      m_2_64 <= N-2
        //      self.0[idx] <= N-1
        // So:
        //      tmp * m_2_64 + self.0[idx]
        //          <= (N-2) * (N-2) + N - 1
        //          = N^2 - 3 * (N - 1)
        //          < N^2
        let tmp = (tmp * m_2_64 + u128::from(self.0[2])) % u128::from(divisor);
        let tmp = (tmp * m_2_64 + u128::from(self.0[1])) % u128::from(divisor);
        let tmp = (tmp * m_2_64 + u128::from(self.0[0])) % u128::from(divisor);
        tmp as u64
    }
}

impl AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for H256 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl FixedHash<U256> for H256 {}

pub struct Blake2b256 {
    inner: blake2b::Blake2b,
}

impl HashAlgo<U256, H256> for Blake2b256 {
    fn create() -> Self {
        let key = b"balloon-blake2b256";
        let inner = blake2b::Blake2bBuilder::new(256 / BYTE_BITS)
            .key(key)
            .build();
        Self { inner }
    }

    fn update<T>(&mut self, data: T) -> Result<()>
    where
        T: AsRef<[u8]>,
    {
        self.inner.update(data.as_ref());
        Ok(())
    }

    fn finalize(self) -> Result<H256> {
        let mut inner: [u8; 32] = unsafe { mem::uninitialized() };
        self.inner.finalize(&mut inner);
        Ok(H256(inner))
    }

    fn finalize_into(self, dst: &mut H256) -> Result<()> {
        self.inner.finalize(dst.as_mut());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::kernel::{FixedUint, U256};
    use slices::u8_slice;

    #[test]
    fn u256_from_slice() {
        let slice =
            u8_slice!("0x123456789abcdef0_13579bdf2468ace0_1122334455667788_99aabbccddeeff00");
        let ffu = U256::from_slice(slice);
        assert_eq!(ffu.0[0], 0x99aa_bbcc_ddee_ff00);
        assert_eq!(ffu.0[1], 0x1122_3344_5566_7788);
        assert_eq!(ffu.0[2], 0x1357_9bdf_2468_ace0);
        assert_eq!(ffu.0[3], 0x1234_5678_9abc_def0);
    }

    #[test]
    fn u256_modulo() {
        let slice =
            u8_slice!("0x123456789abcdef0_13579bdf2468ace0_1122334455667788_99aabbccddeeff00");
        let ffu = U256::from_slice(slice);
        assert_eq!(ffu.modulo(!0), 0xd058_e168_f27b_0258);
    }
}
