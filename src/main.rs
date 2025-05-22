mod custom;
mod deserialization;
mod serialization;

use std::{result, str::Bytes};

use serialization::serialize::serialize::{
    BitVectorTrait, ListTrait, serialize_boolean, serialize_unsigned_integer,
    serialize_vector_of_boolean, serialize_vector_of_unsigned_integers,
};

use deserialization::deserialize::deserialize::{
    BitVectorTraitDeserialize, ListDeserializeTrait, deserialize_boolean,
    deserialize_unsigned_integer, deserialize_vector_of_boolean,
    deserialize_vector_unsigned_integers,
};

use custom::types::types::{BitVector, List};

fn main() {}
