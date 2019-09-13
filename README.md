# rust-triton-clients
Client libraries for Triton APIs

Note: This code still under development and subject to change at any time.

This crate includes:

* Triton client library interface with the following methods:
    * Zone
        * `get_zone_config`
    * SAPI (https://github.com/joyent/sdc-sapi/blob/master/docs/index.md#services)
        * `list_services`
        * `create_service`
        * `get_service`
        * `update_service`
        * `delete_service`


# Build
```
cargo build
```

# Run Example
```
cargo run --example simple
```

# Development
## Testing
```
cargo test
```

or

```
cargo test -- --nocapture
```

## Committing
Before commit, run the following:
```
cargo fmt