# svgview
![crates.io](https://img.shields.io/crates/v/svgview.svg) 
![GitHub release (latest by date)](https://img.shields.io/github/v/release/sdedovic/svgview?color=green&style=plastic)

## Usage
```bash
svgview path/to/some/file.svg
```

## Purpose
I needed a tool to view SVGs while I work on them in other software. This tool should function similarly to `feh`. Requirements:
- display an SVG and allow resizing
- without rasterization artifacts
- and automatically reload on file changes

## TODO/Outstanding Issues
- Nix derivation does not link the binary properly. It will build and execute on my Arch machine with the Nix package manager so long as I am not in a Nix shell when executing.
- Center content in window
- Rasterize the SVG in separate thread for performance and to keep the surface looking correct

## Installing
```bash
cargo install svgview
```

## Building
```bash
cargo build
```
