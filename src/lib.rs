mod deserialization;
mod serialization;

use deserialization::deserialize::deserialize::{
    deserialize_boolean, deserialize_unsigned_integer,
};
use serialization::serialize::serialize::{serialize_boolean, serialize_unsigned_integer};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_unsigned_integer_test() {
        let result = serialize_unsigned_integer(1025u16);

        let test_data = vec!["01", "04"];
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
}
