use crate::custom::types::types::{BitVector, List};

pub mod deserialize {

    use super::*;
    pub trait ListDeserializeTrait {
        fn deserialize(&self) -> Option<Vec<u128>>;
    }

    pub trait BitVectorTraitDeserialize {
        fn deserialize(&self) -> Option<u64>;
    }
    pub fn deserialize_boolean(data: String) -> Option<bool> {
        if data == "01" {
            Some(true)
        } else if data == "00" {
            Some(false)
        } else {
            None
        }
    }

    pub fn deserialize_unsigned_integer(data: Vec<String>) -> Option<u128> {
        let result = data
            .iter()
            .enumerate()
            .map(|(i, j)| u128::from_str_radix(j, 16).unwrap() << (8 * i))
            .sum();

        Some(result)
    }

    pub fn deserialize_vector_unsigned_integers<T>(data: Vec<String>) -> Option<Vec<T>>
    where
        T: From<u128> + Copy,
    {
        let mut result = Vec::new();
        for chunk in data.chunks(8) {
            let num = deserialize_unsigned_integer(chunk.into()).unwrap();
            result.push(T::from(num));
        }

        Some(result)
    }

    pub fn deserialize_vector_of_boolean(data: Vec<String>) -> Option<Vec<bool>> {
        let result = data
            .iter()
            .map(|item| deserialize_boolean(item.to_string()).unwrap())
            .collect();

        Some(result)
    }

    impl<T> ListDeserializeTrait for List<T>
    where
        T: Into<u128> + Copy,
    {
        fn deserialize(&self) -> Option<Vec<u128>> {
            let mut result: Vec<u128> = Vec::new();
            for chunk in self.serialize_data.chunks(8) {
                let num = deserialize_unsigned_integer(chunk.into()).unwrap();
                result.push(num);
            }

            Some(result)
        }
    }

    impl BitVectorTraitDeserialize for BitVector {
        fn deserialize(&self) -> Option<u64> {
            let data = &self.serialize;
            let length: usize = self.length.into();

            let transform_data: Vec<String> = data
                .iter()
                .map(|item| format!("{:b}", u8::from_str_radix(item, 16).unwrap()))
                .collect();

            let concat_data = transform_data.join("");
            let return_data = &concat_data[..length];

            Some(u64::from_str_radix(return_data, 2).unwrap())
        }
    }
}
