mod deserialization;
mod serialization;

use deserialization::deserialize::deserialize::{
    deserialize_boolean, deserialize_unsigned_integer, deserialize_vector_of_boolean,
    deserialize_vector_unsigned_integers,
};
use serialization::serialize::serialize::{
    serialize_boolean, serialize_unsigned_integer, serialize_vector_of_boolean,
    serialize_vector_of_unsigned_integers,
};

#[cfg(test)]
mod tests {
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

        let deserialize: Vec<u128> = deserialize_vector_unsigned_integers(byte_array.unwrap()).unwrap();

        
        assert_eq!(deserialize, data);
    }

    #[test]
    fn deserialize_vector_of_unsigned_integers_should_fail_test() {
        let data: Vec<u128> = vec![256, 512, 768];
        let byte_array = serialize_vector_of_unsigned_integers(data.clone());

        let deserialize: Vec<u128> = deserialize_vector_unsigned_integers(byte_array.unwrap()).unwrap();

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
}
