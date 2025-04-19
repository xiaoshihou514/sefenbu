sefenbu（色分布，sè fēn bù，color distribution）visualizes OKHSV color distribution for an image. The main use for this is to discover the color palette for a given image (to create colorschemes, for example).

![Demo](https://github.com/user-attachments/assets/b480a782-d129-4d35-9ee6-2712e9b2cf8d)

## Usage

Just `sefenbu <your image>`

## Installation

Install dependencies for [bevy](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies) first.

```shell
cargo install sefenbu
```

For those not on wayland, remove the wayland flag from `Cargo.toml` and build from source.