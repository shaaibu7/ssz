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
}
