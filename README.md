# json serialization/deserializaton benchmark

json serialization and deserialization benchmark

- [x] deserialization
 - [x] java
 - [x] dotnet
 - [x] rust

- [ ] serialization

## setup
```bash
./uncompress.sh
```

## prepare json files
a basic small.json file (from https://json.org/example.html) is used to create samples
### generate json files
```
fsi ./setup.fsx
```

### create the setup files
```bash
./compress.sh
```

### Benchmark result

| Lang | Version | Benchmark | time | Maximum resident set size (kbytes) |
|-|-|-|-|-|
| Rust | 1.73.0 | v3 | avg 334ms | 275220KB |
| Rust | 1.73.0 | v2 | avg 3341ms | 2176KB |
| Rust | 1.73.0 | v1 | avg 3772ms | 4484KB |
| Java | 20 | gson | avg 764ms | 121300KB |
| Java | 20 | jackson | avg 869ms | 128952KB |
| Dotnet | 8 | v3 | avg 958.82ms | 87084KB | 