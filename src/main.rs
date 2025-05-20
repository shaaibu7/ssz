mod deserialization;
mod serialization;

use std::result;

use serialization::serialize::serialize::{
    serialize_boolean, serialize_unsigned_integer, serialize_vector_of_boolean,
    serialize_vector_of_unsigned_integers,
};

use deserialization::deserialize::deserialize::{
    deserialize_boolean, deserialize_unsigned_integer, deserialize_vector_of_boolean,
    deserialize_vector_unsigned_integers,
};

fn main() {
    // let serialize = serialize_unsigned_integer(255u16);

    // println!("{:?}", serialize);

    // let deserialize = deserialize_unsigned_integer(serialize.unwrap());

    // println!("{}", deserialize.unwrap());

    // let result = serialize_boolean(false);
    // println!("{}", result.clone().unwrap());

    // let ret = deserialize_boolean(result.unwrap());

    // println!("{}", ret.unwrap());

    // let data = 768;
    // let result = format!("{:02x}", data);

    // println!("{result}")
    let vect: Vec<u32> = vec![256, 512, 768];

    let serialize = serialize_vector_of_unsigned_integers(vect);
    println!("{serialize:?}")

    // let deserialize_vec: Vec<u128> =
    //     deserialize_vector_unsigned_integers(serialize.unwrap()).unwrap();

    // println!("{:?}", deserialize_vec)

    // let data = vec![true, false, false, true, true];
    // let result = serialize_vector_of_boolean(data);

    // let deserialize = deserialize_vector_of_boolean(result.unwrap());

    // println!("{:?}", deserialize)
}
