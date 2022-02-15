# svgview
## Purpose
I needed a tool to view SVGs while I work on them in other software. This tool should function similarly to `feh`. Requirements:
- display an SVG full screen 
- without rasterization artifacts
- and automatically reload on changes

## TODO/Outstanding Issues
- Nix derivation does not link the binary properly. It will build and execute on my Arch machine with the Nix package manager so long as I am not in a Nix shell when executing.
- Center content in window
- Render in separate thread for performance

## Building
```cargo build```

## Installing
```cargo install svgview```
