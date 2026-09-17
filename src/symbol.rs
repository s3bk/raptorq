#[cfg(feature = "std")]
use std::{ops::AddAssign, vec::Vec};

#[cfg(not(feature = "std"))]
use core::ops::AddAssign;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::octet::Octet;
use crate::octets::add_assign;
use crate::octets::fused_addassign_mul_scalar;
use crate::octets::mulassign_scalar;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// Elementary unit of data, for encoding/decoding purposes.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Symbol<T=Vec<u8>> {
    value: T,
}

impl<T> std::ops::Deref for Symbol<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: AsRef<[u8]>> Symbol<T> {
    pub fn new(value: T) -> Symbol<T> {
        Symbol { value }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.value.as_ref().len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.value.as_ref().is_empty()
    }

    /// Return the underlying byte slice for a symbol.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_ref()
    }

    /// Consume a symbol into a vector of bytes.
    #[allow(dead_code)]
    pub fn into_bytes(self) -> Vec<u8> {
        self.value.as_ref().into()
    }
}

impl<T: AsMut<[u8]>> Symbol<T> {
    #[allow(dead_code)]
    #[inline]
    pub fn mulassign_scalar(&mut self, scalar: &Octet) {
        mulassign_scalar(self.value.as_mut(), scalar);
    }

    #[allow(dead_code)]
    #[inline]
    pub fn fused_addassign_mul_scalar<U: AsRef<[u8]>>(&mut self, other: &Symbol<U>, scalar: &Octet) {
        fused_addassign_mul_scalar(self.value.as_mut(), other.value.as_ref(), scalar);
    }
}

impl<'a, T: AsMut<[u8]>, U: AsRef<[u8]>> AddAssign<&'a Symbol<U>> for Symbol<T> {
    #[inline]
    fn add_assign(&mut self, other: &'a Symbol<U>) {
        add_assign(self.value.as_mut(), other.value.as_ref());
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use rand::RngExt;
    use std::vec::Vec;

    use crate::symbol::Symbol;

    #[test]
    fn add_assign() {
        let symbol_size = 41;
        let mut data1: Vec<u8> = vec![0; symbol_size];
        let mut data2: Vec<u8> = vec![0; symbol_size];
        let mut result: Vec<u8> = vec![0; symbol_size];
        for ((d1, d2), res) in data1
            .iter_mut()
            .zip(data2.iter_mut())
            .zip(result.iter_mut())
        {
            *d1 = rand::rng().random();
            *d2 = rand::rng().random();
            *res = *d1 ^ *d2;
        }
        let mut symbol1 = Symbol::new(data1);
        let symbol2 = Symbol::new(data2);

        symbol1 += &symbol2;
        assert_eq!(result, symbol1.into_bytes());
    }
}
