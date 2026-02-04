# Arrowmax

High-performance Arrow data stack: columnar storage, zero-copy streaming, and schema codegen.

## Crates

| Crate | Description |
|-------|-------------|
| [minarrow](minarrow/) | Apache Arrow-compatible columnar data library with SIMD alignment and minimal dependencies |
| [lightstream](lightstream/) | Zero-copy Arrow IPC streaming with async support and memory-mapping |
| [flatarrow](flatarrow/) | Arrow schema DSL with codegen and wire format (WIP) |

## License

Copyright Peter Garfield Bower 2025-2026.

Released under MIT. See LICENSE for details.

## Acknowledgements
`Arrowmax` includes a from-scratch implementation of the *Apache Arrow* memory layout, under `Minarrow`, inspired by the standards pioneered by *Apache Arrow*, *Arrow2*, and *Polars*. Additionally, an extensive set of native stream-friendly connectors under the `Lightstream` package to streamline the process of working with live data effectively in Rust.

`Arrowmax` is not affiliated with *Apache Arrow*.