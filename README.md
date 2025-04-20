<!-- sefenbu（色分布，sè fēn bù，color distribution）visualizes color distribution for an image. The main use for this is to make colorschemes. Supports OKHSV, OKHSL, HSLuv, HSL and HSV. -->

sefenbu（色分布，sè fēn bù，color distribution）visualizes color distribution for an image. The main use for this is to make colorschemes. Supports OKHSV, OKHSL.

![Demo](https://github.com/user-attachments/assets/b480a782-d129-4d35-9ee6-2712e9b2cf8d)

## Usage

```
Usage: sefenbu [OPTIONS] <FILE>

Arguments:
  <FILE>  Input image

Options:
  -u, --using <USING>  Color space
  -h, --help           Print help
  -V, --version        Print version
```

## Installation

Install dependencies for [bevy](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies) first.

```shell
cargo install sefenbu
```

For those on X11, remove the wayland flag from `Cargo.toml` and build from source.

## Credits

- @TD-Sky for all my rust refactoring questions
