use crate::custom::types::types::List;

pub mod serialize {
    use super::*;

    pub trait ListTrait {
        fn serialize(&mut self) -> Option<Vec<String>>;
        // fn deserialize(&self) -> Option<Vec<u128>>;
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
}
