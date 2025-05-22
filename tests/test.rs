use ssz::custom::types::types::{BitVector, List};
use ssz::deserialization::deserialize::deserialize::{
    BitVectorTraitDeserialize, ListDeserializeTrait, deserialize_boolean,
    deserialize_unsigned_integer, deserialize_vector_of_boolean,
    deserialize_vector_unsigned_integers,
};
use ssz::serialization::serialize::serialize::{
    BitVectorTrait, ListTrait, serialize_boolean, serialize_unsigned_integer,
    serialize_vector_of_boolean, serialize_vector_of_unsigned_integers,
};

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn serialize_unsigned_integer_test() {
        let result = serialize_unsigned_integer(1025u16);

        let test_data = vec!["01", "04", "00", "00", "00", "00", "00", "00"];
        assert_eq!(result.unwrap(), test_data);
    }

    #[test]
    fn serialize_boolean_with_true_should_pass_test() {
        let result = serialize_boolean(true);

        assert_eq!(result.unwrap(), "01");
    }

    #[test]
    fn serialize_boolean_with_true_should_fail_test() {
        let result = serialize_boolean(true);

        assert_ne!(result.unwrap(), "00");
    }

    #[test]
    fn serialize_boolean_with_false_should_pass_test() {
        let result = serialize_boolean(false);

        assert_eq!(result.unwrap(), "00");
    }

    #[test]
    fn serialize_boolean_with_false_should_fail_test() {
        let result = serialize_boolean(false);

        assert_ne!(result.unwrap(), "01");
    }

    #[test]
    fn deserialize_unsigned_integer_should_pass_test() {
        let serialize_data = serialize_unsigned_integer(255u8);

        let deserialize_result = deserialize_unsigned_integer(serialize_data.unwrap());

        assert_eq!(deserialize_result.unwrap(), 255);
    }

    #[test]
    fn deserialize_unsigned_integer_should_fail_test() {
        let serialize_data = serialize_unsigned_integer(255u8);

        let deserialize_result = deserialize_unsigned_integer(serialize_data.unwrap());

        assert_ne!(deserialize_result.unwrap(), 1025);
    }

    #[test]
    fn deserialize_boolean_should_pass_test() {
        let serialize_data = serialize_boolean(true);

        let deserialize_result = deserialize_boolean(serialize_data.unwrap());

        assert_eq!(deserialize_result.unwrap(), true);

        let serialize_data = serialize_boolean(false);

        let deserialize_result = deserialize_boolean(serialize_data.unwrap());

        assert_eq!(deserialize_result.unwrap(), false)
    }

    #[test]
    fn deserialize_boolean_should_fail_test() {
        let serialize_data = serialize_boolean(true);

        let deserialize_result = deserialize_boolean(serialize_data.unwrap());

        assert_ne!(deserialize_result.unwrap(), false);

        let serialize_data = serialize_boolean(false);

        let deserialize_result = deserialize_boolean(serialize_data.unwrap());

        assert_ne!(deserialize_result.unwrap(), true);
    }

    #[test]
    fn serialize_vector_of_unsigned_integers_should_pass_test() {
        let data: Vec<u16> = vec![256, 512, 768];
        let byte_array = serialize_vector_of_unsigned_integers(data);

        let result = vec![
            "00", "01", "00", "00", "00", "00", "00", "00", "00", "02", "00", "00", "00", "00",
            "00", "00", "00", "03", "00", "00", "00", "00", "00", "00",
        ];
        assert_eq!(byte_array.unwrap(), result)
    }

    #[test]
    fn serialize_vector_of_unsigned_integers_should_fail_test() {
        let data: Vec<u16> = vec![256, 512, 768];
        let byte_array = serialize_vector_of_unsigned_integers(data);

        let result = vec![
            "000", "01", "00", "00", "00", "00", "00", "00", "00", "02", "00", "00", "00", "00",
            "00", "00", "00", "03", "00", "00", "00", "00", "00", "00",
        ];
        assert_ne!(byte_array.unwrap(), result)
    }

    #[test]
    fn serialize_vector_of_boolean_should_pass_test() {
        let data: Vec<bool> = vec![true, false, true, true];
        let byte_array = serialize_vector_of_boolean(data);

        let result = vec!["01", "00", "01", "01"];
        assert_eq!(byte_array.unwrap(), result)
    }

    #[test]
    fn serialize_vector_of_boolean_should_fail_test() {
        let data: Vec<bool> = vec![true, false, true, true];
        let byte_array = serialize_vector_of_boolean(data);

        let result = vec!["01", "01", "01", "01"];
        assert_ne!(byte_array.unwrap(), result)
    }

    #[test]
    fn deserialize_vector_of_unsigned_integers_should_pass_test() {
        let data: Vec<u128> = vec![256, 512, 768];
        let byte_array = serialize_vector_of_unsigned_integers(data.clone());

        let deserialize: Vec<u128> =
            deserialize_vector_unsigned_integers(byte_array.unwrap()).unwrap();

        assert_eq!(deserialize, data);
    }

    #[test]
    fn deserialize_vector_of_unsigned_integers_should_fail_test() {
        let data: Vec<u128> = vec![256, 512, 768];
        let byte_array = serialize_vector_of_unsigned_integers(data.clone());

        let deserialize: Vec<u128> =
            deserialize_vector_unsigned_integers(byte_array.unwrap()).unwrap();

        let data: Vec<u128> = vec![256, 512, 768, 1025];

        assert_ne!(deserialize, data);
    }

    #[test]
    fn deserialize_vector_of_boolean_should_pass_test() {
        let data: Vec<bool> = vec![true, false, true, true];
        let byte_array = serialize_vector_of_boolean(data.clone());

        let deserialize = deserialize_vector_of_boolean(byte_array.unwrap());

        assert_eq!(deserialize.unwrap(), data)
    }

    #[test]
    fn deserialize_vector_of_boolean_should_fail_test() {
        let data: Vec<bool> = vec![true, false, true, true];
        let byte_array = serialize_vector_of_boolean(data.clone());

        let deserialize = deserialize_vector_of_boolean(byte_array.unwrap());

        let data: Vec<bool> = vec![true, false, true, true, false];
        assert_ne!(deserialize.unwrap(), data)
    }

    #[test]
    fn serialize_list_should_pass_test() {
        let mut list: List<u16> = List {
            length: 5,
            data: vec![1024, 2048, 3072],
            serialize_data: Vec::new(),
        };

        let serialize = list.serialize();
        let expected_result = vec![
            "00", "04", "00", "00", "00", "00", "00", "00", "00", "08", "00", "00", "00", "00",
            "00", "00", "00", "0c", "00", "00", "00", "00", "00", "00",
        ];

        assert_eq!(serialize.unwrap(), expected_result);
    }

    #[test]
    fn serialize_list_should_fail_test() {
        let mut list: List<u16> = List {
            length: 5,
            data: vec![1024, 2048, 3072],
            serialize_data: Vec::new(),
        };

        let serialize = list.serialize();
        let expected_result = vec![
            "00", "01", "00", "00", "00", "00", "00", "00", "00", "08", "00", "00", "00", "00",
            "00", "00", "00", "0c", "00", "00", "00", "00", "00", "00",
        ];

        assert_ne!(serialize.unwrap(), expected_result);
    }

    #[test]
    fn deserialize_list_should_pass_test() {
        let data: Vec<u128> = vec![1024, 2048, 3072];
        let mut list: List<u128> = List {
            length: 5,
            data: data,
            serialize_data: Vec::new(),
        };

        let serialize = list.serialize();
        let deserialize = list.deserialize();

        assert_eq!(deserialize.unwrap(), list.data);
    }

    #[test]
    fn deserialize_list_should_fail_test() {
        let data: Vec<u16> = vec![1024, 2048, 3072];
        let mut list: List<u16> = List {
            length: 5,
            data: data,
            serialize_data: Vec::new(),
        };

        let serialize = list.serialize();
        let deserialize = list.deserialize();

        let mock_data: Vec<u128> = vec![1024, 2048, 3073];
        assert_ne!(deserialize.unwrap(), mock_data);
    }

    #[test]
    fn serialize_bit_vector_should_pass_test() {
        let mut data = BitVector {
            length: 10,
            data: 1011010010,
            serialize: Vec::new(),
        };

        let serialize_data = data.serialize();
        let result = vec!["b4", "80"];

        assert_eq!(serialize_data.unwrap(), result);
    }

    #[test]
    fn serialize_bit_vector_should_fail_test() {
        let mut data = BitVector {
            length: 10,
            data: 1011010010,
            serialize: Vec::new(),
        };

        let serialize_data = data.serialize();
        let result = vec!["B4", "80"];

        assert_ne!(serialize_data.unwrap(), result);
    }

    #[test]
    fn deserialize_bit_vector_should_pass_test() {
        let mut data = BitVector {
            length: 10,
            data: 1011010010,
            serialize: Vec::new(),
        };

        let serialize_data = data.serialize();
        let deserialize_data = data.deserialize();

        assert_eq!(format!("{:b}", deserialize_data.unwrap()), "1011010010");
        assert_eq!(deserialize_data.unwrap(), 722); // 722 is 1011010010 in decimal
    }

    #[test]
    fn deserialize_bit_vector_should_fail_test() {
        let mut data = BitVector {
            length: 10,
            data: 1011010010,
            serialize: Vec::new(),
        };

        let serialize_data = data.serialize();
        let deserialize_data = data.deserialize();

        assert_ne!(format!("{:b}", deserialize_data.unwrap()), "10110100100");
        assert_ne!(deserialize_data.unwrap(), 723);
    }
}
