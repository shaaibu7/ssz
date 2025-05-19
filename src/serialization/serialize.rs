pub mod serialize {
    pub fn serialize_unsigned_integer<T>(num: T) -> Option<Vec<String>>
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

        Some(result.to_vec())
    }

    pub fn serialize_boolean(data: bool) -> Option<String> {
        if data {
            Some(format!("{:02x}", 01))
        } else {
            Some(format!("{:02x}", 00))
        }
    }
}
