# Numbers

| Bits         | Unsigned int | Signed int | Floating |
| ------------ | ------------ | ---------- | -------- |
| 8            | u8           | i8         |          |
| 16           | u16          | i16        |          |
| 32           | u32          | i32        | f32      |
| 64           | u64          | i64        | f64      |
| 128          | u128         | i128       |          |
| machine word | usize        | isize      |          |

- machine word is 32 bit in 32 bit machines and 64 bit in 64 bit machines
- Literals can have a suffix with the type
- Literals can have arbitrary `_` separators not affecting the value
- Inference of literals is deferred as soon as they are used

| Literal       | Type            | Decimal value |
| ------------- | --------------- | ------------- |
| `116i8`       | i8              | 116           |
| `0xcafeu32`   | u32             | 51966         |
| `0b0010_1010` | inferred binary | 42            |
| `0o106`       | inferred octal  | 70            |
