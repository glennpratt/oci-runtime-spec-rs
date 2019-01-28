# oci-runtime-spec 
## Open Container Runtime Specs

OCI Runtime Spec[1] serde[2] types for Rust translated from JSON Schema by
QuickType[3].

  [1]: https://github.com/opencontainers/runtime-spec "OCI Runtime Spec project"
  [2]: https://serde.rs/ "Serde"
  [3]: https://quicktype.io/ "QuickType"

## Update schema
```
quicktype -s schema ../runtime-spec/schema/*.json -o src/schema.rs --derive-debug --visibility public --density dense
```
