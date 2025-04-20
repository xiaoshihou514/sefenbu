![Crates.io Version](https://img.shields.io/crates/v/sefenbu)

[中文文档](./README-zh.md)

sefenbu（色分布，sè fēn bù，color distribution）visualizes color distribution for an image. The main use for this is to make colorschemes. Supports OKHSV, OKHSL, HSV, HSL.

|                                `sefenbu -u okhsv test.jpg`                                |                                `sefenbu -u okhsl test.jpg`                                |
| :---------------------------------------------------------------------------------------: | :---------------------------------------------------------------------------------------: |
| ![okhsv](https://github.com/user-attachments/assets/b480a782-d129-4d35-9ee6-2712e9b2cf8d) | ![okhsl](https://github.com/user-attachments/assets/3ee76175-b631-4162-bc9f-8121644ade14) |
|                                 `sefenbu -u hsv test.jpg`                                 |                                 `sefenbu -u hsl test.jpg`                                 |
|  ![hsv](https://github.com/user-attachments/assets/f30a24dd-dcbc-4c94-b64f-df51226bf179)  |  ![hsl](https://github.com/user-attachments/assets/6d682129-6cf7-488d-9923-e31ead506800)  |

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

You also need the `shaders/` directory during runtime.

## Installation

Install dependencies for [bevy](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies) first.

```shell
cargo install sefenbu
```

For those on X11, remove the wayland flag from `Cargo.toml` and build from source.

## Credits

- [TD-Sky](https://github.com/TD-Sky) for all my rust refactoring questions
- [char-BS](https://github.com/char-BS) for bearing with my enthusiasm
