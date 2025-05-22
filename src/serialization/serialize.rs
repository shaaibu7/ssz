use crate::custom::types::types::{BitVector, List};

pub mod serialize {
    use super::*;

    pub trait ListTrait {
        fn serialize(&mut self) -> Option<Vec<String>>;
    }

    pub trait BitVectorTrait {
        fn serialize(&mut self) -> Option<Vec<String>>;
    }

    pub fn serialize_unsigned_integer<T>(num: T) -> Option<Vec<String>>
    where
        T: Into<u128> + Copy,
    {
        let data_bytes = num.into().to_le_bytes();

        let length_of_data_bytes = std::mem::size_of::<T>();

        let trimmed_size_data = &data_bytes[..8];

        let result: Vec<String> = trimmed_size_data
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        Some(result.to_vec())
    }

    pub fn serialize_boolean(data: bool) -> Option<String> {
        if data {
            Some(format!("{:02x}", 01))
        } else {
            Some(format!("{:02x}", 00))
        }
    }

    pub fn serialize_vector_of_unsigned_integers<T>(data: Vec<T>) -> Option<Vec<String>>
    where
        T: Into<u128> + Copy,
    {
        let result = data
            .iter()
            .flat_map(|item| serialize_unsigned_integer(*item).unwrap())
            .collect();

        Some(result)
    }

    pub fn serialize_vector_of_boolean(data: Vec<bool>) -> Option<Vec<String>> {
        let result = data
            .iter()
            .map(|item| serialize_boolean(*item).unwrap())
            .collect();

        Some(result)
    }

    impl<T> ListTrait for List<T>
    where
        T: Into<u128> + Copy,
    {
        fn serialize(&mut self) -> Option<Vec<String>> {
            let result: Vec<String> = self
                .data
                .iter()
                .flat_map(|item| serialize_unsigned_integer(*item).unwrap())
                .collect();

            self.serialize_data = result.clone();
            Some(result)
        }
    }

    fn padding_bits(data: u64) -> String {
        let mut binary = format!("{}", data);
        let pad_value = binary.len() % 8;

        if pad_value != 0 {
            let padding = 8 - pad_value;
            binary.extend(std::iter::repeat('0').take(padding));
        }

        binary
    }

    impl BitVectorTrait for BitVector {
        fn serialize(&mut self) -> Option<Vec<String>> {
            let mut data: String = self.data.to_string();

            if self.data.to_string().len() % 8 != 0 {
                data = padding_bits(self.data);
            }

            let result: Vec<String> = data
                .as_bytes()
                .chunks(8)
                .map(|chunk| {
                    let chunk_str = std::str::from_utf8(chunk).unwrap();

                    let num = u8::from_str_radix(chunk_str, 2).unwrap();

                    format!("{:02x}", num)
                })
                .collect();
            self.serialize = result.clone();
            Some(result)
        }
    }
}
