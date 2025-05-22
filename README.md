# 🦀 ssz-lite-rs – Lightweight SSZ Serialization in Rust

**`ssz-lite-rs`** is a minimal and extensible implementation of Ethereum’s Simple Serialize (SSZ) specification, written in Rust. It provides core utilities for serializing and deserializing primitive SSZ types in a lightweight, idiomatic way for Rust developers building Ethereum-compatible clients, tooling, and infrastructure.

---

## ✨ Features

### ✅ Implemented

- **Unsigned Integers**: Supports `u8`, `u16`, `u32`, `u64`, and `u128`. All integers are encoded using little-endian byte order as specified in SSZ.
- **Booleans**: Encoded as a single byte — `0x01` for `true`, `0x00` for `false`.
- **Fixed-size Vectors**: Homogeneous sequences of elements of known, constant length. Suitable for arrays where the element count is static.
- **Variable-size Lists**: Dynamically-sized homogeneous sequences. Encoded with proper offset handling, following SSZ’s rule for lists.
- **Bit Vectors**: Space-efficient binary representation of fixed-length bit sequences. Internally packed into bytes with the least significant bit first.

---

## 🔮 Roadmap

The following features are planned for future releases to bring the implementation closer to full SSZ compatibility:

- **Bit Lists**: Variable-length bit sequences that terminate with a special end marker. Unlike bit vectors, bit lists support dynamic sizing while remaining compact.
- **Containers**: Composite types made of multiple fields of different types, similar to Rust structs. Container serialization involves concatenating field encodings in defined order.
- **Union Types**: Encodings for values that may be one of several types, each identified by a selector byte. Commonly used in beacon chain type definitions.
- **Merkleization**: Generation of Merkle roots from serialized data structures for use in light clients, consensus proofs, and state transition verification.
- **Consensus Object Support**: Planned support for Ethereum consensus-layer types such as `BeaconBlock`, `Attestation`, and `Validator`, enabling plug-and-play SSZ support for consensus clients.

---

## 🧪 Testing

The project includes comprehensive unit tests for all implemented types, validating serialization and deserialization consistency, adherence to the SSZ format, and handling of edge cases like overflows and malformed inputs.

---

## 🤝 Contributing

Contributions are highly welcome! The project is designed with modularity and extensibility in mind. You can contribute by implementing additional SSZ types, fixing bugs, improving performance, writing more tests, or improving documentation.

Before contributing, please ensure your changes are thoroughly tested and conform to the project's design principles. Open an issue or discussion to align on direction before submitting a large pull request.

---

## 📝 License

This project is licensed under the MIT License, allowing reuse within both open and proprietary projects, with proper attribution.
