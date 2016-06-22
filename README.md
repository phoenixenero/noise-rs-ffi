# An FFI for `noise-rs`

A C FFI for [`noise-rs`](https://github.com/bjz/noise-rs). If you just want to generate noise in Rust, just use `noise-rs` directly.

## Documentation

See `src/*`.

## Implemented functionality

- Perlin noise 2 - 4
- OpenSimplex noise 2 - 3
- Cell noise (also: Worley noise, Voronoi noise)
    - Value 2 - 4
    - Range 2 - 4
    - Range Inv 2 - 4
- Cell noise with Manhattan distance
    - Value 2 - 4
    - Range 2 - 4
    - Range Inv 2 - 4

## License

This library is dual-licensed as MIT and Apache v2.0, the same as the Rust language itself. You may choose from both depending on your needs.
