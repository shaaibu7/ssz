pub mod deserialize {
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
        let result = data.iter()
            .enumerate()
            .map(|(i, j)| u128::from_str_radix(j, 16).unwrap() << (8 * i))
            .sum();

        Some(result)
    }
}
