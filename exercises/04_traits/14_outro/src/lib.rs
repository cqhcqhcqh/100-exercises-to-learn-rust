// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    pub value: u16
}

impl SaturatingU16 {
    // pub fn new(value: u16) -> Self {
    //     SaturatingU16 { value: value }
    // }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        return Self { value: value };
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        return Self { value: *value };
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        return Self { value: {value as u16} };
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        return Self { value: {*value as u16} };
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        return SaturatingU16 { value: self.value.saturating_add(rhs.value) }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        return SaturatingU16 { value: self.value.saturating_add(rhs.value) }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> Self::Output {
        return SaturatingU16 { value: self.value.saturating_add(rhs) }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &u16) -> Self::Output {
        // let clone_self = self.clone();
        return SaturatingU16 { value: self.value.saturating_add(*rhs) }
    }
}

// impl PartialEq for SaturatingU16 {
//     fn eq(&self, other: &Self) -> bool {
//         return self.value == other.value
//     }
// }

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        return self.value == *other
    }
}