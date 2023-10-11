# tsid
TSID generator for rust, this library is reimplementation of java [tsid-creator](https://github.com/f4b6a3/tsid-creator/) created by Fabio Lima

![example workflow](https://github.com/jakudlaty/tsid/actions/workflows/rust.yml/badge.svg)

Goals:
- [x] Generate TSID with simple, naive implementation
- [ ] Add basic tests to check compatibility
- [ ] Benchmarks (compare the speed to java library)
- [x] Publish first usable version to crates.io
- [x] GitHub actions workflow to CI
- [x] Serde serialization
- [x] Thread safety

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)

## How to use it?
```rust
use tsid::create_tsid;

let tsid = create_tsid()
```

Crate features:
- ```display``` - add Display trait to TSID
- ```debug``` - adds debug trait to TSID
- ```serde``` - adds serde dependency and serialization/deserialization
- ```serde_as_string``` - tells serde to serialize TSID as string (disabling this feature means that serde will serialize to string for human readable formats ant tu u64 otherwise)
- ```bson``` - adds From bson conversion
- ```bson_as_string``` - the same as for serde
