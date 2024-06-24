// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(_value: u32) -> WrappingU32 {
        WrappingU32 { value: _value }
    }
}

// 如果 where WrappingU32: From<u32> trait
// 编译器会为 u32 实现 Into<WrappingU32> trait
// impl Into<WrappingU32> for u32 where WrappingU32: From<u32> {
//     fn into(self) -> WrappingU32 {
//         WrappingU32::from(self)
//     }
// }

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
