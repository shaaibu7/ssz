pub mod types {
    pub struct List<T> {
        pub length: u16,
        pub data: Vec<T>,
        pub serialize_data: Vec<String>,
    }

    pub struct BitVector {
        pub length: u16,
        pub data: u64,
        pub serialize: Vec<String>,
    }
}
