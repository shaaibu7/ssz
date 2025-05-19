mod serialization;

use serialization::serialize::serialize::{
    deserialize_unsigned_integer, serialize_unsigned_integer,
};

fn main() {
    let serialize = serialize_unsigned_integer(1025u16);

    println!("{:?}", serialize);

    let deserialize = deserialize_unsigned_integer(serialize);

    println!("{}", deserialize);
}
