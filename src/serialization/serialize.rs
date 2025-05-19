pub mod serialize {
    pub fn serialize_unsigned_integer<T>(num: T) -> Vec<String>
    where
        T: Into<u128> + Copy,
    {
        let data_bytes = num.into().to_le_bytes();

        let length_of_data_bytes = std::mem::size_of::<T>();

        let trimmed_size_data = &data_bytes[..length_of_data_bytes];

        let result: Vec<String> = trimmed_size_data
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        result.to_vec()
    }

    pub fn deserialize_unsigned_integer(data: Vec<String>) -> u128 {
        data.iter()
            .enumerate()
            .map(|(i, j)| u128::from_str_radix(j, 16).unwrap() << (8 * i))
            .sum()
    }
}
