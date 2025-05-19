mod deserialization;
mod serialization;

use serialization::serialize::serialize::{serialize_boolean, serialize_unsigned_integer};

use deserialization::deserialize::deserialize::{
    deserialize_boolean, deserialize_unsigned_integer,
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
}
