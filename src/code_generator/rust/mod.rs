use std::{
    cmp::max,
    collections::HashMap,
    hash::Hash,
    io::{Read, Write},
};

use crate::errors::JuteError;

pub mod utilities;
pub mod writer;

pub trait JuteSerializable: Sized {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<(), JuteError>;
    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, JuteError>;
}

impl JuteSerializable for i32 {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<(), JuteError> {
        out.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, JuteError> {
        let mut arr = [0u8; 4];
        bytes.read_exact(&mut arr)?;
        Ok(i32::from_be_bytes(arr))
    }
}

impl JuteSerializable for i64 {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<(), JuteError> {
        out.write_all(&self.to_be_bytes())?;
        Ok(())
    }

    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, JuteError> {
        let mut arr = [0u8; 8];
        bytes.read_exact(&mut arr)?;
        Ok(i64::from_be_bytes(arr))
    }
}

impl JuteSerializable for f32 {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<(), JuteError> {
        out.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, JuteError> {
        let mut arr = [0u8; 4];
        bytes.read_exact(&mut arr)?;
        Ok(f32::from_be_bytes(arr))
    }
}

impl JuteSerializable for f64 {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<(), JuteError> {
        out.write_all(&self.to_be_bytes())?;

        Ok(())
    }
    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, JuteError> {
        let mut arr = [0u8; 8];
        bytes.read_exact(&mut arr)?;
        Ok(f64::from_be_bytes(arr))
    }
}

impl JuteSerializable for bool {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<(), JuteError> {
        out.write_all(if *self { &[0x01] } else { &[0x00] })?;
        Ok(())
    }
    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, JuteError> {
        let mut byte = [0u8; 1];
        bytes.read_exact(&mut byte)?;
        Ok(byte[0] != 0)
    }
}

impl JuteSerializable for u8 {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<(), JuteError> {
        out.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, JuteError> {
        let mut byte = [0u8; 1];
        bytes.read_exact(&mut byte)?;
        Ok(byte[0])
    }
}
impl<T> JuteSerializable for Vec<T>
where
    T: JuteSerializable,
{
    fn serialize<W: Write>(&self, out: &mut W) -> Result<(), JuteError> {
        let elem_count = self.len() as i32;
        out.write_all(&elem_count.to_be_bytes())?;
        for elem in self {
            elem.serialize(out)?;
        }
        Ok(())
    }
    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, JuteError> {
        let mut arr = [0u8; 4];
        bytes.read_exact(&mut arr)?;
        let elem_count = max(i32::from_be_bytes(arr), 0) as usize;
        let mut vec = Vec::<T>::with_capacity(elem_count);
        for _i in 0..elem_count {
            vec.push(T::deserialize(bytes)?);
        }
        Ok(vec)
    }
}
impl JuteSerializable for String {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<(), JuteError> {
        let bytes = self.as_bytes();
        out.write_all(&(bytes.len() as i32).to_be_bytes())?;
        out.write_all(bytes)?;
        Ok(())
    }
    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, JuteError> {
        let mut arr = [0u8; 4];
        bytes.read_exact(&mut arr)?;
        let elem_count = max(i32::from_be_bytes(arr), 0) as usize;

        let mut buffer = vec![0u8; elem_count];
        bytes.read_exact(&mut buffer)?;
        let result = String::from_utf8(buffer)?;
        Ok(result)
    }
}
impl<T, U> JuteSerializable for HashMap<T, U>
where
    T: JuteSerializable + Hash + Eq,
    U: JuteSerializable,
{
    fn serialize<W: Write>(&self, out: &mut W) -> Result<(), JuteError> {
        let elem_count = self.len() as i32;
        out.write_all(&elem_count.to_be_bytes())?;
        for (key, value) in self {
            key.serialize(out)?;
            value.serialize(out)?;
        }
        Ok(())
    }
    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, JuteError> {
        let mut arr = [0u8; 4];
        bytes.read_exact(&mut arr)?;
        let elem_count = max(i32::from_be_bytes(arr), 0) as usize;
        let mut map = HashMap::<T, U>::with_capacity(elem_count);
        for _i in 0..elem_count {
            map.insert(T::deserialize(bytes)?, U::deserialize(bytes)?);
        }
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip<T: JuteSerializable + PartialEq + std::fmt::Debug>(value: T) {
        let mut buf = vec![];
        value.serialize(&mut buf).unwrap();

        let mut cursor = std::io::Cursor::new(buf);
        let decoded = T::deserialize(&mut cursor).unwrap();

        assert_eq!(value, decoded);
    }

    #[test]
    fn test_i32() {
        roundtrip(123i32);
        roundtrip(-999i32);
    }

    #[test]
    fn test_string() {
        roundtrip("hello".to_string());
        roundtrip("".to_string());
    }

    #[test]
    fn test_bool() {
        roundtrip(true);
        roundtrip(false);
    }

    #[test]
    fn test_vec() {
        roundtrip(vec![1i32, 2, 3]);
        roundtrip::<Vec<i32>>(vec![]);
    }

    #[test]
    fn test_map() {
        let mut m = HashMap::new();
        m.insert("a".to_string(), 1i32);
        m.insert("b".to_string(), 2i32);
        roundtrip(m);
    }
}
